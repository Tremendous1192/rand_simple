// 乱数の種を返すメソッドのテスト
#[test]
fn test_get_seed() {
    use crate::Uniform;

    let uniform = Uniform::new(1192u32);
    let seed: u32 = uniform.get_seed();

    assert_eq!(seed, 1192u32);
}

#[test]
fn test_sample() {
    use crate::Uniform;

    let uniform = Uniform::new(1192u32);
    let next_1: f64 = uniform.sample(); // 0.8698977918526851
    let next_2: f64 = uniform.sample(); // 0.8698962295590659
    let next_3: f64 = uniform.sample(); // 0.8694609875952501
    let next_4: f64 = uniform.sample(); // 0.028744847520427975
    let next_5: f64 = uniform.sample(); // 0.33302628396382233
    
    assert_eq!((next_1, next_2, next_3, next_4, next_5),
        (0.8698977918526851f64, 0.8698962295590659f64, 0.8694609875952501f64, 0.028744847520427975f64, 0.33302628396382233f64));
}