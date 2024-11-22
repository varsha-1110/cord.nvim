#[derive(Debug)]
pub struct LogEvent {
    pub message: String,
}

use crate::messages::events::event::{EventContext, OnEvent};

impl OnEvent for LogEvent {
    fn on_event(self, _ctx: &EventContext) -> crate::Result<()> {
        println!("{}", self.message);
        Ok(())
    }
}

impl LogEvent {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
