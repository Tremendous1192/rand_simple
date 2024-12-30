#[test]
fn test_sample() {
    let mut generator = crate::Uniform::new(1192u32);

    assert_eq!(generator.sample(), 0.8512317447111084f64);
    assert_eq!(generator.sample(), 0.11755379967334535f64);
    assert_eq!(generator.sample(), 0.563863670584714f64);
    assert_eq!(generator.sample(), 0.5643948017536651f64);
    assert_eq!(generator.sample(), 0.10760219257967597f64);

    assert_eq!(generator.sample(), 0.7706018015208193f64);
    assert_eq!(generator.sample(), 0.623794760933098f64);
    assert_eq!(generator.sample(), 0.9329806673184458f64);
    assert_eq!(generator.sample(), 0.6701782226260236f64);
    assert_eq!(generator.sample(), 0.3726202066458343f64);
}
/* 
#[test]
fn test_constructor_macro() {
    let mut uniform = crate::create_uniform!(1192u32);
    assert_eq!(uniform.sample(), 0.8512317447111084f64);

    // 基準時刻からの経過時間のミリ秒を乱数の種とした場合
    let mut uniform_2 = crate::create_uniform!();
    let next_2: f64 = uniform_2.sample();
    println!("{}", next_2);
}
*/