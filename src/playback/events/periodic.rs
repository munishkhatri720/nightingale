use std::sync::Arc;
use songbird::{Event, EventContext, EventHandler};
use crate::api::session::Session;
use crate::channel::Sender;

pub struct PeriodicMetrics {
    #[allow(unused)]
    session: Arc<Session>,
    #[allow(unused)]
    sender: Sender
}

impl PeriodicMetrics {
    pub async fn new(session: Arc<Session>) -> Self {
        let sender = session.playback.sharder.sender.clone();

        Self {
            session,
            sender
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for PeriodicMetrics {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        let _ = ctx;
        None
    }
}
