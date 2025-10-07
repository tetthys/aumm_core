use serde::{ Deserialize, Serialize };
use crate::key::KeyId;

/// Tap bucket – non-overlapping by design.
/// These correspond to "very-short", "short", and "normal" taps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum TapKind {
    VeryShort,
    Short,
    Normal,
}

/// High-level gesture types the recognizer can emit.
/// Designed to be orthogonal: a key at a given time
/// produces exactly one of these gestures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Gesture {
    Tap(TapKind),
    DoubleTap,
    TripleTap,
    Hold,
}

/// Recognized gesture bound to a specific key and its decision timestamp.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GestureEvent {
    pub key: KeyId,
    pub gesture: Gesture,
    pub decided_at_ms: u64,
}

impl GestureEvent {
    /// Helper for quickly constructing in tests.
    pub fn new<K: Into<KeyId>>(key: K, gesture: Gesture, decided_at_ms: u64) -> Self {
        Self {
            key: key.into(),
            gesture,
            decided_at_ms,
        }
    }
}
