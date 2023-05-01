use std::time::{SystemTime, UNIX_EPOCH}; // 時刻の取得

#[macro_export]
/// 一様乱数のインスタンスを生成するマクロ
macro_rules! uniform {
    // 乱数の種を指定せずに、インスタンスを生成する
    () => {{
        let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH).expect("Time went backwards")
        .as_nanos() % 1000_000_000;
        Uniform {xyzw: set_state(nanos as u32),}
    }};
    // 乱数の種を指定して、インスタンスを生成する
    ($seed:expr) => {
        Uniform {xyzw: set_state($seed as u32),}
    };
}