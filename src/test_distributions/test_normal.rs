#[test]
fn test_sample() {
    let generator = crate::Normal::new(1192u32, 765u32);

    assert_eq!(generator.sample(), -1.2296205447119757f64);
    assert_eq!(generator.sample(), -1.2239488495150759f64);
    assert_eq!(generator.sample(), -0.010954509460085884f64);
    assert_eq!(generator.sample(), 0.847978078473172f64);
    assert_eq!(generator.sample(), 0.5819869085530331f64);

    assert_eq!(generator.sample(), -1.1219620171039946f64);
    assert_eq!(generator.sample(), 0.44522382606948707f64);
    assert_eq!(generator.sample(), 0.4452233453037017f64);
    assert_eq!(generator.sample(), -1.0518408413093652f64);
    assert_eq!(generator.sample(), -1.4147664638570046f64);
}
