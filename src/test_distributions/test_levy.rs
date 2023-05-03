#[test]
fn test_sample() {
    let generator = crate::Levy::new(1192u32, 765u32);

    assert_eq!(generator.sample(), 0.15611801640551176f64);
    assert_eq!(generator.sample(), 0.15611651161177115f64);
    assert_eq!(generator.sample(), 0.6666068601508252f64);
    assert_eq!(generator.sample(), 0.6635342559821038f64);
    assert_eq!(generator.sample(), 0.15719508774760568f64);

    assert_eq!(generator.sample(), 0.1571962312057757f64);
    assert_eq!(generator.sample(), 0.723872098277674f64);
    assert_eq!(generator.sample(), 7.575136136442043f64);
    assert_eq!(generator.sample(), 0.5995696051010854f64);
    assert_eq!(generator.sample(), 0.7929363470126304f64);
}