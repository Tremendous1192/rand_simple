#[test]
fn test_sample() {
    use crate::Cauchy;

    let cauchy = Cauchy::new(1192u32, 765u32);
    let next_1: f64 = cauchy.sample(); // 1.0046339315561652
    let next_2: f64 = cauchy.sample(); // -0.01291838755998273
    let next_3: f64 = cauchy.sample(); // -0.5187224698169873
    let next_4: f64 = cauchy.sample(); // 1.000001079830585
    let next_5: f64 = cauchy.sample(); // -0.5786371516784212

    assert_eq!((next_1, next_2, next_3, next_4, next_5),
        (1.0046339315561652f64, -0.01291838755998273f64, -0.5187224698169873f64, 1.000001079830585f64, 0.7434731230777029f64));
}
