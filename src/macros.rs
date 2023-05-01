#[macro_export]
/// 一様乱数のインスタンスを生成するマクロ
macro_rules! create_uniform {
    // 引数無し
    () => {{
        use std::time::{SystemTime, UNIX_EPOCH}; // 時刻の取得
        let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH).expect("Time went backwards")
        .as_nanos() % 1000_000_000;
        $crate::Uniform::new(nanos as u32)
    }};
    // 引数有り
    ($seed:expr) => {
        $crate::Uniform::new($seed as u32)
    };
}