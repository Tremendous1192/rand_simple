#[test]
fn test_sample() {
    use crate::Geometric;

    let geometric = Geometric::new(1192u32);
    let next_1: u32 = geometric.sample(0.5f64);
    let next_2: u32 = geometric.sample(0.5f64);
    let next_3: u32 = geometric.sample(0.5f64);
    let next_4: u32 = geometric.sample(0.5f64);
    let next_5: u32 = geometric.sample(0.5f64);
    
    assert_eq!((next_1, next_2, next_3, next_4, next_5),
        (4u32, 1u32, 1u32, 1u32, 4u32));
}