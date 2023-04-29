#[test]
fn test_sample() {
    use crate::Normal;

    let normal = Normal::new(1192u32, 765u32);
    let next_1: f64 = normal.sample(); // -1.2296205447119757
    let next_2: f64 = normal.sample(); // -1.2239488495150759
    let next_3: f64 = normal.sample(); // -0.010954509460085884
    let next_4: f64 = normal.sample(); // 0.847978078473172
    let next_5: f64 = normal.sample(); // 0.5819869085530331

    assert_eq!((next_1, next_2, next_3, next_4, next_5),
        (-1.2296205447119757f64, -1.2239488495150759f64, -0.010954509460085884f64, 0.847978078473172f64, 0.5819869085530331f64));
}
