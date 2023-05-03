#[test]
fn test_sample() {
    let generator = crate::HalfCauchy::new(1192u32, 765u32);

    assert_eq!(generator.sample(), 0.9999951805774843f64);
    assert_eq!(generator.sample(), 0.997692678256128f64);
    assert_eq!(generator.sample(), 1.0000036370605054f64);
    assert_eq!(generator.sample(), 3.2349256430464535f64);
    assert_eq!(generator.sample(), 1.1500040204613344f64);

    assert_eq!(generator.sample(), 4.180015813756806f64);
    assert_eq!(generator.sample(), 1.0074269792982646f64);
    assert_eq!(generator.sample(), 1.0077822523388844f64);
    assert_eq!(generator.sample(), 1.0588897399529487f64);
    assert_eq!(generator.sample(), 0.30202564055362213f64);
}
