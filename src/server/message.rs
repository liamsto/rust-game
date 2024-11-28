use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::battle_multiplayer::BattleState; // Import BattleState for use in MessageKind

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageKind {
    Action {
        name: String,
        user: String,
        target: String,
    },
    Event {
        code: u8,
    },
    Text {
        text: String,
    },
    BattleUpdate {
        state: BattleState,
    },
}

impl Display for MessageKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MessageKind::Action { name, user, target } => {
                write!(f, "Action: {} used {} on {}", user, name, target)
            }
            MessageKind::Event { code } => {
                write!(f, "Event: Code {}", code)
            }
            MessageKind::Text { text } => {
                write!(f, "Text: {}", text)
            }
            MessageKind::BattleUpdate { .. } => {
                write!(f, "BattleUpdate")
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub kind: MessageKind,
}

impl Message {
    /// Creates a new Message with the given MessageKind.
    pub fn new(kind: MessageKind) -> Message {
        Message { kind }
    }

    /// Creates a new BattleUpdate message with the given BattleState.
    pub fn new_battle_update(state: &BattleState) -> Message {
        Message {
            kind: MessageKind::BattleUpdate {
                state: state.clone(),
            },
        }
    }

    /// Serializes the Message into a JSON byte vector.
    pub fn serialize(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Deserializes a Message from a JSON byte slice.
    pub fn deserialize(bytes: &[u8]) -> Result<Message, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}
