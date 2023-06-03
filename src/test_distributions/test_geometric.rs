#[test]
fn test_sample() {
    let generator = crate::Geometric::new(1192u32);

    assert_eq!(generator.sample(0.5f64), 2u64);
    assert_eq!(generator.sample(0.5f64), 3u64);
    assert_eq!(generator.sample(0.5f64), 5u64);
    assert_eq!(generator.sample(0.5f64), 1u64);
    assert_eq!(generator.sample(0.5f64), 1u64);

    assert_eq!(generator.sample(0.5f64), 1u64);
    assert_eq!(generator.sample(0.5f64), 1u64);
    assert_eq!(generator.sample(0.5f64), 3u64);
    assert_eq!(generator.sample(0.5f64), 1u64);
    assert_eq!(generator.sample(0.5f64), 4u64);
}