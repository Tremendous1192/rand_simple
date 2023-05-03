#[test]
fn test_sample() {
    let generator = crate::HalfNormal::new(1192u32, 765u32);

    assert_eq!(generator.sample(), 2.5308912695634582f64);
    assert_eq!(generator.sample(), 2.5309034670566124f64);
    assert_eq!(generator.sample(), 1.224799810880738f64);
    assert_eq!(generator.sample(), 1.22763235370392f64);
    assert_eq!(generator.sample(), 2.5222057693132345f64);

    assert_eq!(generator.sample(), 2.522196595931608f64);
    assert_eq!(generator.sample(), 1.1753550604082077f64);
    assert_eq!(generator.sample(), 0.363332944896171f64);
    assert_eq!(generator.sample(), 1.2914577291803542f64);
    assert_eq!(generator.sample(), 1.1230027949487293f64);
}
