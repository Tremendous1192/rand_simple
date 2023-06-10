#[test]
fn test_sample() {
    let generator = crate::Levy::new(1192u32, 765u32);

    assert_eq!(generator.sample(), 0.27866346364478645f64);
    assert_eq!(generator.sample(), 0.2786618619526834f64);
    assert_eq!(generator.sample(), 2.2087768571350024f64);
    assert_eq!(generator.sample(), 2.2087795622729307f64);
    assert_eq!(generator.sample(), 2.2092594730473043f64);

    assert_eq!(generator.sample(), 2.2147000059108906f64);
    assert_eq!(generator.sample(), 0.2655714496629292f64);
    assert_eq!(generator.sample(), 0.2655696303429814f64);
    assert_eq!(generator.sample(), 3.9885073025301105f64);
    assert_eq!(generator.sample(), 3.988506007356597f64);
}