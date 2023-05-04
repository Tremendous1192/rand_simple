#[test]
fn test_sample() {
    let generator = crate::Laplace::new(1192u32);

    assert_eq!(generator.sample(), -1.2961143823579562f64);
    assert_eq!(generator.sample(), -1.2961132994585387f64);
    assert_eq!(generator.sample(), -1.2958116127184585f64);
    assert_eq!(generator.sample(), 0.019924410014410185f64);
    assert_eq!(generator.sample(), 0.9239834103418245f64);

    assert_eq!(generator.sample(), 0.02036710671999963f64);
    assert_eq!(generator.sample(), 4.50171657701542f64);
    assert_eq!(generator.sample(), -0.4536120985143202f64);
    assert_eq!(generator.sample(), -3.4528184449477815f64);
    assert_eq!(generator.sample(), -1.261661443713526f64);
}