#[test]
fn test_sample() {
    let generator = crate::Bernoulli::new(1192u32);

    assert_eq!(generator.sample(0.5f64), 0u32);
    assert_eq!(generator.sample(0.5f64), 0u32);
    assert_eq!(generator.sample(0.5f64), 0u32);
    assert_eq!(generator.sample(0.5f64), 1u32);
    assert_eq!(generator.sample(0.5f64), 1u32);

    assert_eq!(generator.sample(0.5f64), 1u32);
    assert_eq!(generator.sample(0.5f64), 1u32);
    assert_eq!(generator.sample(0.5f64), 0u32);
    assert_eq!(generator.sample(0.5f64), 0u32);
    assert_eq!(generator.sample(0.5f64), 0u32);
}