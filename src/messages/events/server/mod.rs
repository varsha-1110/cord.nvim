pub mod disconnect;
pub mod log;
pub mod ready;

pub use disconnect::DisconnectEvent;
pub use log::LogEvent;
pub use ready::ReadyEvent;

use crate::messages::events::event::{EventContext, OnEvent};

#[derive(Debug)]
pub enum ServerEvent {
    Ready(ReadyEvent),
    Log(LogEvent),
    #[allow(dead_code)]
    Disconnect(DisconnectEvent),
}

impl OnEvent for ServerEvent {
    fn on_event(self, ctx: &mut EventContext) -> crate::Result<()> {
        match self {
            Self::Ready(e) => e.on_event(ctx),
            Self::Log(e) => e.on_event(ctx),
            Self::Disconnect(e) => e.on_event(ctx),
        }
    }
}
