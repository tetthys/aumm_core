// Placeholder for future time abstraction / monotonic clock sources.
// For now we accept timestamps as parameters (deterministic & testable).

/// Marker trait for future time providers.
pub trait TimeSource {}
