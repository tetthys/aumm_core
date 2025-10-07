use serde::{ Deserialize, Serialize };
use crate::key::KeyId;

/// Low-level input state change for a key/button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyState {
    Down,
    Up,
}

/// A key press/release with an absolute timestamp in milliseconds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEvent {
    pub key: KeyId,
    pub state: KeyState,
    pub ts_ms: u64,
}

/// Feed this into the recognizer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputEvent {
    Key(KeyEvent),
    /// Let recognizer process time-based decisions (e.g., finalizing single/double).
    /// Supply the current timestamp you want the engine to evaluate against.
    Tick(u64),
}
