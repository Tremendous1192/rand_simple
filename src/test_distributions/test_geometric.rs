#[test]
fn test_sample() {
    let generator = crate::Geometric::new(1192u32);

    assert_eq!(generator.sample(), 2u64);
    assert_eq!(generator.sample(), 3u64);
    assert_eq!(generator.sample(), 5u64);
    assert_eq!(generator.sample(), 1u64);
    assert_eq!(generator.sample(), 1u64);

    assert_eq!(generator.sample(), 1u64);
    assert_eq!(generator.sample(), 1u64);
    assert_eq!(generator.sample(), 3u64);
    assert_eq!(generator.sample(), 1u64);
    assert_eq!(generator.sample(), 4u64);
}