#[test]
fn test_sample() {
    let generator = crate::Weibull::new(1192u32);

    assert_eq!(generator.sample(), 1.5180935542424843f64);
    assert_eq!(generator.sample(), 0.5301830109445694f64);
    assert_eq!(generator.sample(), 1.209226668295876f64);
    assert_eq!(generator.sample(), 1.2077540600068124f64);
    assert_eq!(generator.sample(), 0.543978867742604f64);

    assert_eq!(generator.sample(), 1.9652008965709442f64);
    assert_eq!(generator.sample(), 1.0430624030844131f64);
    assert_eq!(generator.sample(), 2.1295625449795437f64);
    assert_eq!(generator.sample(), 0.914460140296167f64);
    assert_eq!(generator.sample(), 0.17658588924749685f64);
}