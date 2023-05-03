#[test]
fn test_sample() {
    let generator = crate::Geometric::new(1192u32);

    assert_eq!(generator.sample(0.5f64), 4u32);
    assert_eq!(generator.sample(0.5f64), 1u32);
    assert_eq!(generator.sample(0.5f64), 1u32);
    assert_eq!(generator.sample(0.5f64), 1u32);
    assert_eq!(generator.sample(0.5f64), 4u32);

    assert_eq!(generator.sample(0.5f64), 6u32);
    assert_eq!(generator.sample(0.5f64), 1u32);
    assert_eq!(generator.sample(0.5f64), 2u32);
    assert_eq!(generator.sample(0.5f64), 4u32);
    assert_eq!(generator.sample(0.5f64), 4u32);
}