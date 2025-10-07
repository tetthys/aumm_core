use thiserror::Error;

#[derive(Debug, Error)]
pub enum AummError {
    #[error("invalid thresholds (ordering violated)")]
    InvalidThresholds,
}
