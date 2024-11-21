use std::env::var;
use std::io;
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

use crate::ipc::discord::client::{Connection, RichClient};

impl Connection for RichClient {
    fn connect(client_id: u64) -> io::Result<Self> {
        let dirs = ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"]
            .iter()
            .filter_map(|&dir| var(dir).ok())
            .chain(["/tmp".to_string()])
            .flat_map(|base| {
                [
                    base.to_string(),
                    format!("{}/app/com.discordapp.Discord", base),
                    format!("{}/snap.discord", base),
                ]
            });

        for dir in dirs {
            for i in 0..10 {
                match UnixStream::connect(format!("{dir}/discord-ipc-{i}")) {
                    Ok(pipe) => {
                        return Ok(RichClient {
                            client_id,
                            pipe: Some(pipe),
                            last_activity: None,
                            pid: std::process::id(),
                        })
                    }
                    Err(e) => match e.kind() {
                        io::ErrorKind::NotFound => continue,
                        _ => return Err(e),
                    },
                }
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "Pipe not found"))
    }

    fn close(&mut self) {
        if let Some(pipe) = self.pipe.take() {
            let _ = pipe.shutdown(Shutdown::Both);
        }
    }
}
