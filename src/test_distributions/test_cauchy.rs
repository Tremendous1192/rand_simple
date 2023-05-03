#[test]
fn test_sample() {
    let generator = crate::Cauchy::new(1192u32, 765u32);

    assert_eq!(generator.sample(), 1.0046339315561652f64);
    assert_eq!(generator.sample(), -0.01291838755998273f64);
    assert_eq!(generator.sample(), -0.5187224698169873f64);
    assert_eq!(generator.sample(), 1.000001079830585f64);
    assert_eq!(generator.sample(), 0.7434731230777029f64);

    assert_eq!(generator.sample(), -0.6817314400265051f64);
    assert_eq!(generator.sample(), 1.0501095022891225f64);
    assert_eq!(generator.sample(), 1.1568758507071866f64);
    assert_eq!(generator.sample(), 0.8493711210561783f64);
    assert_eq!(generator.sample(), -1.852043618007516f64);
}
