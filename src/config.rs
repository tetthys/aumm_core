use serde::{ Deserialize, Serialize };

/// Timing thresholds in milliseconds.
/// Ensure strict ordering: t_vs < t_s < t_n < t_h
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Thresholds {
    /// Very-short tap maximum (exclusive upper bound of VS bucket).
    pub t_vs: u64,
    /// Short tap maximum (exclusive).
    pub t_s: u64,
    /// Normal tap maximum (exclusive). Above this and below `t_h` is still considered "tap-normal".
    pub t_n: u64,
    /// Hold threshold (inclusive): durations >= t_h are considered hold.
    pub t_h: u64,
    /// Inter-tap interval maximum (inclusive) to aggregate multi-taps.
    pub t_d: u64,
}

impl Thresholds {
    pub fn sanity_check(&self) -> bool {
        self.t_vs < self.t_s && self.t_s < self.t_n && self.t_n < self.t_h && self.t_d > 0
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self { t_vs: 80, t_s: 200, t_n: 400, t_h: 500, t_d: 500 }
    }
}
