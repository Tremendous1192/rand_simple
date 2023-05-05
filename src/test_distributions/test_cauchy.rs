#[test]
fn test_sample() {
    let generator = crate::Cauchy::new(1192u32, 765u32);

    assert_eq!(generator.sample(), 0.9999997103138784f64);
    assert_eq!(generator.sample(), 1.000005406668742f64);
    assert_eq!(generator.sample(), 1.0108892830219072f64);
    assert_eq!(generator.sample(), 0.9973826967958467f64);
    assert_eq!(generator.sample(), 0.9999991818616032f64);

    assert_eq!(generator.sample(), -20.694885029180362f64);
    assert_eq!(generator.sample(), -0.6787853733324916f64);
    assert_eq!(generator.sample(), 0.9983898371145459f64);
    assert_eq!(generator.sample(), 1.0000006343935983f64);
    assert_eq!(generator.sample(), -2.3225478104889734f64);
}
