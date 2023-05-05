#[test]
fn test_sample() {
    let generator = crate::HalfNormal::new(1192u32, 765u32);

    assert_eq!(generator.sample(), 1.8943489630074781f64);
    assert_eq!(generator.sample(), 1.8943544071672804f64);
    assert_eq!(generator.sample(), 0.6728590194747661f64);
    assert_eq!(generator.sample(), 0.672858607442474f64);
    assert_eq!(generator.sample(), 0.6727855219499161f64);

    assert_eq!(generator.sample(), 0.6719586464278682f64);
    assert_eq!(generator.sample(), 1.9404806118269244f64);
    assert_eq!(generator.sample(), 1.9404872585753394f64);
    assert_eq!(generator.sample(), 0.5007198451442664f64);
    assert_eq!(generator.sample(), 0.500719926442756f64);
}
