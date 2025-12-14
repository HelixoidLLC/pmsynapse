use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    NodeCreated,
    NodeUpdated,
    EdgeCreated,
    SessionStatusChanged,
    // Future: Claude session events
}

#[derive(Clone, Debug, Serialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: serde_json::Value,
}

pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub fn publish(&self, event_type: EventType, data: serde_json::Value) {
        let event = Event {
            event_type,
            timestamp: chrono::Utc::now(),
            data,
        };
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
