#[test]
fn test_sample() {
    let generator = crate::Laplace::new(1192u32);

    assert_eq!(generator.sample(), -0.7214398300954756f64);
    assert_eq!(generator.sample(), -0.7214484932908147f64);
    assert_eq!(generator.sample(), -0.7238619872114567f64);
    assert_eq!(generator.sample(), 0.6134495405023045f64);
    assert_eq!(generator.sample(), 0.9258988839847477f64);

    assert_eq!(generator.sample(), 0.6116787536799468f64);
    assert_eq!(generator.sample(), 4.423200638601012f64);
    assert_eq!(generator.sample(), -0.264993147622555f64);
    assert_eq!(generator.sample(), -2.906158844204326f64);
    assert_eq!(generator.sample(), -0.997063339250916f64);
}