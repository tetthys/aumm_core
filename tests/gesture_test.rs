use aumm_core::{ Gesture, TapKind, gesture::GestureEvent, KeyId };

#[test]
fn tapkind_buckets_are_distinct() {
    assert_ne!(TapKind::VeryShort, TapKind::Short);
    assert_ne!(TapKind::Short, TapKind::Normal);
}

#[test]
fn gesture_event_new_works() {
    let ge = GestureEvent::new("K", Gesture::Tap(TapKind::Short), 1234);
    assert_eq!(ge.key, KeyId("K".into()));
    match ge.gesture {
        Gesture::Tap(TapKind::Short) => {}
        _ => panic!("expected tap-short"),
    }
    assert_eq!(ge.decided_at_ms, 1234);
}
