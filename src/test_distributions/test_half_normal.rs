#[test]
fn test_sample() {
    use crate::HalfNormal;

    let normal = HalfNormal::new(1192u32, 765u32);
    let next_1: f64 = normal.sample(); // 2.5308912695634582
    let next_2: f64 = normal.sample(); // 2.5309034670566124
    let next_3: f64 = normal.sample(); // 1.224799810880738
    let next_4: f64 = normal.sample(); // 1.22763235370392
    let next_5: f64 = normal.sample(); // 2.5222057693132345

    assert_eq!((next_1, next_2, next_3, next_4, next_5),
        (2.5308912695634582f64, 2.5309034670566124f64, 1.224799810880738f64, 1.22763235370392f64, 2.5222057693132345f64));
}
