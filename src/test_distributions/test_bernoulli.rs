#[test]
fn test_sample() {
    use crate::Bernoulli;

    let bernoulli = Bernoulli::new(1192u32);
    let next_1: u32 = bernoulli.sample(0.5f64); // 0.8698977918526851 -> 0u32
    let next_2: u32 = bernoulli.sample(0.5f64); // 0.8698962295590659 -> 0u32
    let next_3: u32 = bernoulli.sample(0.5f64); // 0.8694609875952501 -> 0u32
    let next_4: u32 = bernoulli.sample(0.5f64); // 0.028744847520427975 -> 1u32
    let next_5: u32 = bernoulli.sample(0.5f64); // 0.33302628396382233 -> 1u32
    
    assert_eq!((next_1, next_2, next_3, next_4, next_5),
        (0u32, 0u32, 0u32, 1u32, 1u32));
}