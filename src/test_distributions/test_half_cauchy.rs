#[test]
fn test_sample() {
    let generator = crate::HalfCauchy::new(1192u32, 765u32);

    assert_eq!(generator.sample(), 0.9999971261133705f64);
    assert_eq!(generator.sample(), 1.0000006123608847f64);
    assert_eq!(generator.sample(), 1.001230545252217f64);
    assert_eq!(generator.sample(), 0.9999965747013356f64);
    assert_eq!(generator.sample(), 0.9999998376367998f64);

    assert_eq!(generator.sample(), 2.6883468716739234f64);
    assert_eq!(generator.sample(), 1.000551624185551f64);
    assert_eq!(generator.sample(), 0.9999995197148379f64);
    assert_eq!(generator.sample(), 1.186710771584489f64);
    assert_eq!(generator.sample(), 0.33121880315616975f64);
}
