use crate::{Bernoulli, set_state, update_and_uniform};

impl Bernoulli {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzw: set_state(_seed),
        }
    }

    /// ある確率の事象が生じたか(1)、否か(0)を返す
    /// * `probability` - ある事象が生じる確率
    pub fn sample(&self, probability: f64) -> u32 {
        if update_and_uniform(&self.xyzw) <= probability { 1u32 }
        else { 0u32 }
    }
}

#[macro_export]
/// ベルヌーイ分布のインスタンスを生成するマクロ
macro_rules! create_bernoulli {
    // 引数無し
    () => {{
        $crate::Bernoulli::new($crate::create_seed())
    }};
    // 引数有り
    ($seed: expr) => {
        $crate::Bernoulli::new($seed as u32)
    };
}