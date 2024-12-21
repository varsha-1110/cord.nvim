local logger = require 'cord.util.logger'
local utils = require 'cord.util'
local spawn = require 'cord.core.spawn'

local uv = vim.loop or vim.uv

local IPC = {}
local mt = { __index = IPC }

function IPC.new(config)
  local self = setmetatable({}, mt)
  self.config = config
  self.pipe = nil
  return self
end

function IPC:connect(callback)
  if self.config.advanced.server.pipe_path then
    self.path = self.config.advanced.server.pipe_path
  else
    self.path = (utils.os_name == 'Windows' and '\\\\.\\pipe\\' or '/tmp/')
      .. 'cord-ipc'
  end
  local pipe = uv.new_pipe()
  self.pipe = pipe

  pipe:connect(self.path, function(err)
    if err then
      if err == 'ENOENT' then
        spawn.spawn_server(self, function() self:connect(callback) end)
        return
      else
        logger.error('Failed to connect to pipe: ' .. err)
      end
      return
    end

    logger.debug('Connected to pipe: ' .. self.path)

    if callback then callback() end
  end)
end

function IPC:read_start(callback)
  if not self.pipe then return end

  self.pipe:read_start(function(err, chunk)
    if err then
      logger.error('Read error: ' .. err)
      return
    end

    if chunk then
      if callback then callback(chunk) end
    else
      self:close()
    end
  end)
end

function IPC:write(data, callback)
  if not self.pipe then return false end

  self.pipe:write(data, function(err)
    if err then
      logger.error('Write error: ' .. err)
      return
    end

    if callback then callback() end
  end)

  return true
end

function IPC:on_close(callback) self.on_close_cb = callback end

function IPC:close()
  if self.pipe then
    logger.debug 'Connection closed'
    self.pipe:read_stop()
    self.pipe:close()
    self.pipe = nil

    if self.on_close_cb then self.on_close_cb() end
  end
end

return IPC
