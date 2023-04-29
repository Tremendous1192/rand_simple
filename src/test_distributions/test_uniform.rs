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

#[test]
fn test_sample_vec() {
    use crate::Uniform;
    let uniform = Uniform::new(1192u32);

    let mut vec_left: Vec<f64> = Vec::<f64>::new();
    let mut vec_right: Vec<f64> = Vec::<f64>::new();

    for _ in 0..10 {
        vec_left.push(uniform.sample());
    }
    vec_right.push(0.8698977918526851f64);
    vec_right.push(0.8698962295590659f64);
    vec_right.push(0.8694609875952501f64);
    vec_right.push(0.028744847520427975f64);
    vec_right.push(0.33302628396382233f64);

    vec_right.push(0.029383523862199747f64);
    vec_right.push(0.4946041811477868f64);
    vec_right.push(0.6544239250138457f64);
    vec_right.push(0.9813640476161065f64);
    vec_right.push(0.8201927081263142f64);

    assert_eq!(vec_left, vec_right);
}