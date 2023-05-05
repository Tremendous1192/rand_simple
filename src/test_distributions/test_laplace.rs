#[test]
fn test_sample() {
    let generator = crate::Laplace::new(1192u32);

    assert_eq!(generator.sample(), -0.824946373682539f64);
    assert_eq!(generator.sample(), 0.3672188413291935f64);
    assert_eq!(generator.sample(), -0.5160794877359308f64);
    assert_eq!(generator.sample(), -0.514606879446867f64);
    assert_eq!(generator.sample(), 0.3948105549252627f64);

    assert_eq!(generator.sample(), -1.272053716010999f64);
    assert_eq!(generator.sample(), -0.3499152225244678f64);
    assert_eq!(generator.sample(), -1.4364153644195985f64);
    assert_eq!(generator.sample(), -0.22131295973622161f64);
    assert_eq!(generator.sample(), 0.7063435569899874f64);
}