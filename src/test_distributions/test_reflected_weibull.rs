#[test]
fn test_sample() {
    let generator = crate::ReflectedWeibull::new(1192u32);

    assert_eq!(generator.sample(), 1.2122183368953001f64);
    assert_eq!(generator.sample(), -1.4477120000678736f64);
    assert_eq!(generator.sample(), 0.13665322180288003f64);
    assert_eq!(generator.sample(), 0.13787177390305932f64);
    assert_eq!(generator.sample(), -1.5361670737675837f64);

    assert_eq!(generator.sample(), 0.7791487469435436f64);
    assert_eq!(generator.sample(), 0.2844732553712587f64);
    assert_eq!(generator.sample(), 2.009626973167161f64);
    assert_eq!(generator.sample(), 0.4160556584149296f64);
    assert_eq!(generator.sample(), -0.2940484102548753f64);
}