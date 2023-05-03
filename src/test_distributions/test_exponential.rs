#[test]
fn test_sample() {
    let generator = crate::Exponential::new(1192u32);

    assert_eq!(generator.sample(), 1.4145870106554208f64);
    assert_eq!(generator.sample(), 1.4145956738507601f64);
    assert_eq!(generator.sample(), 1.417009167771402f64);
    assert_eq!(generator.sample(), 0.653298360531125f64);
    assert_eq!(generator.sample(), 0.23147472099618702f64);

    assert_eq!(generator.sample(), 0.652412967119946f64);
    assert_eq!(generator.sample(), 0.007480193248447539f64);
    assert_eq!(generator.sample(), 0.9581403281825003f64);
    assert_eq!(generator.sample(), 3.5993060247642714f64);
    assert_eq!(generator.sample(), 1.6902105198108612f64);
}