#[test]
fn test_sample() {
    let generator = crate::Rayleigh::new(1192u32);

    assert_eq!(generator.sample(), 1.6820148695272708f64);
    assert_eq!(generator.sample(), 1.6820200200061592f64);
    assert_eq!(generator.sample(), 1.683454286739858f64);
    assert_eq!(generator.sample(), 1.1430646180606983f64);
    assert_eq!(generator.sample(), 0.6804038815235948f64);

    assert_eq!(generator.sample(), 1.1422897768254305f64);
    assert_eq!(generator.sample(), 0.12231265877616707f64);
    assert_eq!(generator.sample(), 1.384297892928036f64);
    assert_eq!(generator.sample(), 2.683022931234197f64);
    assert_eq!(generator.sample(), 1.8385921352006602f64);
}