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
        // step 1: 区間[0, 1]の一様乱数uを発生させる
        // step 2: u ≦ θ(発生確率)のとき1を所望の乱数として出力する
        // u > θのときは0を出力する
        if update_and_uniform(&self.xyzw) <= probability { 1u32 }
        else { 0u32 }
    }
}

#[macro_export]
/// ベルヌーイ分布のインスタンスを生成するマクロ
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
macro_rules! create_bernoulli {
    () => {
        $crate::Bernoulli::new($crate::create_seed())
    };
    ($seed: expr) => {
        $crate::Bernoulli::new($seed as u32)
    };
}