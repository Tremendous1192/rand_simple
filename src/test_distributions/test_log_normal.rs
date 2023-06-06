#[test]
fn test_sample() {
    let generator = crate::LogNormal::new(1192u32, 765u32);

    assert_eq!(generator.sample(), 1.1216353517595588f64);
    assert_eq!(generator.sample(), 1.1216353890566597f64);
    assert_eq!(generator.sample(), 6.359976970424311f64);
    assert_eq!(generator.sample(), 6.359913355601059f64);
    assert_eq!(generator.sample(), 6.413688007714963f64);

    assert_eq!(generator.sample(), 6.286568867094377f64);
    assert_eq!(generator.sample(), 2.0720289603372675f64);
    assert_eq!(generator.sample(), 2.0759940193976307f64);
    assert_eq!(generator.sample(), 4.257700544355279f64);
    assert_eq!(generator.sample(), 4.2577055908488655f64);
}
