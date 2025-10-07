use aumm_core::Thresholds;

#[test]
fn default_thresholds_are_sane() {
    let th = Thresholds::default();
    assert!(th.sanity_check(), "default thresholds must be strictly ordered and valid");
}

#[test]
fn invalid_threshold_order_is_caught_by_sanity_check() {
    // t_vs >= t_s -> invalid
    let th = Thresholds { t_vs: 100, t_s: 100, t_n: 300, t_h: 500, t_d: 250 };
    assert!(!th.sanity_check());
}

#[test]
fn custom_thresholds_ok_when_strictly_increasing() {
    let th = Thresholds { t_vs: 60, t_s: 120, t_n: 250, t_h: 600, t_d: 300 };
    assert!(th.sanity_check());
}
