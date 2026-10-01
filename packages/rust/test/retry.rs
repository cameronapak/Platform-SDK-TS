#[test]
fn retry_backoff_preserves_native_delays_and_saturates_without_overflow() {
    for (attempt, milliseconds) in [
        (0, 100),
        (2, 400),
        (8, 25_600),
        (9, 30_000),
        (63, 30_000),
        (u32::MAX, 30_000),
    ] {
        assert_eq!(
            HttpClient::retry_delay(attempt),
            std::time::Duration::from_millis(milliseconds)
        );
    }
}
