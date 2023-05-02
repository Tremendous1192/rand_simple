use crate::{Uniform, set_state, update_and_uniform};

impl Uniform {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzw: set_state(_seed),
        }
    }

    /// 閉区間[0, 1]の乱数を返す
    pub fn sample(&self) -> f64 {
        update_and_uniform(&self.xyzw)
    }
}


#[macro_export]
/// 一様乱数のインスタンスを生成するマクロ
macro_rules! create_uniform {
    // 引数無し
    () => {{
        $crate::Uniform::new($crate::create_seed())
    }};
    // 引数有り
    ($seed: expr) => {
        $crate::Uniform::new($seed as u32)
    };
}