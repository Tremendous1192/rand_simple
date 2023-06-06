#[test]
fn test_sample() {
    let generator = crate::Exponential::new(1192u32);

    assert_eq!(generator.sample(), 1.9053655174552453f64);
    assert_eq!(generator.sample(), 0.12505745483565f64);
    assert_eq!(generator.sample(), 0.8298004023628254f64);
    assert_eq!(generator.sample(), 0.8310189544630047f64);
    assert_eq!(generator.sample(), 0.11384327337397195f64);

    assert_eq!(generator.sample(), 1.4722959275034888f64);
    assert_eq!(generator.sample(), 0.9776204359312041f64);
    assert_eq!(generator.sample(), 2.702774153727106f64);
    assert_eq!(generator.sample(), 1.109202838974875f64);
    assert_eq!(generator.sample(), 0.4662031907028283f64);
}