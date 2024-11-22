#[derive(Debug, Default)]
pub struct DisconnectEvent;

use crate::messages::events::event::{EventContext, OnEvent};

impl DisconnectEvent {
    pub fn on_disconnect(self) {}
}

impl OnEvent for DisconnectEvent {
    fn on_event(self, _ctx: &EventContext) -> crate::Result<()> {
        Ok(())
    }
}
