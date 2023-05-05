#[test]
fn test_sample() {
    let generator = crate::Normal::new(1192u32, 765u32);

    assert_eq!(generator.sample(), 0.11478775584530312f64);
    assert_eq!(generator.sample(), 0.11478778909773256f64);
    assert_eq!(generator.sample(), 1.8500247563430081f64);
    assert_eq!(generator.sample(), 1.8500147539260656f64);
    assert_eq!(generator.sample(), 1.8584344576985734f64);

    assert_eq!(generator.sample(), 1.8384154317504018f64);
    assert_eq!(generator.sample(), 0.72852830129631f64);
    assert_eq!(generator.sample(), 0.7304400844698349f64);
    assert_eq!(generator.sample(), 1.4487292362281599f64);
    assert_eq!(generator.sample(), 1.4487304214901442f64);
}
