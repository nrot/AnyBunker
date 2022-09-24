use std::fmt::Display;

use crate::schemes;


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Events{
    Ticker{
        id: i64
    },
    NewMessage{
        index: String,
        message: schemes::LogMessage
    },
    ReIndex{
        index: String
    },
    NewReport{
        index: uuid::Uuid
    }
}

pub type SenderEvents = tokio::sync::broadcast::Sender<Events>;
pub type ReceiverEvents = tokio::sync::broadcast::Receiver<Events>;

impl Display for Events{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self {
            Events::Ticker { id } => format!("Ticker: {}", id),
            Events::NewMessage { index, message } => format!("New Message index: {} message:{}", index, message),
            Events::ReIndex { index } => format!("Reindex index {}", index),
            Events::NewReport { index } => format!("New report {}", index),
        };
        write!(f, "Event: {}", m)
    }
}