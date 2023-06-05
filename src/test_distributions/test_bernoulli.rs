#[test]
fn test_sample() {
    let generator = crate::Bernoulli::new(1192u32);

    assert_eq!(generator.sample(), 0u64);
    assert_eq!(generator.sample(), 1u64);
    assert_eq!(generator.sample(), 0u64);
    assert_eq!(generator.sample(), 0u64);
    assert_eq!(generator.sample(), 1u64);

    assert_eq!(generator.sample(), 0u64);
    assert_eq!(generator.sample(), 0u64);
    assert_eq!(generator.sample(), 0u64);
    assert_eq!(generator.sample(), 0u64);
    assert_eq!(generator.sample(), 1u64);
}