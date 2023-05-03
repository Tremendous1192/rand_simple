#[test]
fn test_sample() {
    let generator = crate::Uniform::new(1192u32);

    assert_eq!(generator.sample(), 0.8698977918526851f64);
    assert_eq!(generator.sample(), 0.8698962295590659f64);
    assert_eq!(generator.sample(), 0.8694609875952501f64);
    assert_eq!(generator.sample(), 0.028744847520427975f64);
    assert_eq!(generator.sample(), 0.33302628396382233f64);

    assert_eq!(generator.sample(), 0.029383523862199747f64);
    assert_eq!(generator.sample(), 0.4946041811477868f64);
    assert_eq!(generator.sample(), 0.6544239250138457f64);
    assert_eq!(generator.sample(), 0.9813640476161065f64);
    assert_eq!(generator.sample(), 0.8201927081263142f64);
}

#[test]
fn test_constructor_macro() {
    let uniform = crate::create_uniform!(1192u32);
    assert_eq!(uniform.sample(), 0.8698977918526851f64);

    // 基準時刻からの経過時間のミリ秒を乱数の種とした場合
    let uniform_2 = crate::create_uniform!();
    let next_2: f64 = uniform_2.sample();
    println!("{}", next_2);
}