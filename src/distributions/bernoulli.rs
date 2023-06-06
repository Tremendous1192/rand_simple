use crate::{Bernoulli, initialize, update};
use std::cell::Cell;

impl Bernoulli {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzw: (u32, u32, u32, u32) = initialize(_seed);
        Self {
            x: Cell::new(xyzw.0), y: Cell::new(xyzw.1), z: Cell::new(xyzw.2), w: Cell::new(xyzw.3),
            probability: Cell::new(0.5f64),
        }
    }

    /// ある確率の事象が生じたか(1u64)、否か(0u64)を返す
    pub fn sample(&self) -> u64 {
        // step 1: 区間[0, 1]の一様乱数uを発生させる
        // step 2: u ≦ θ(発生確率)のとき1を所望の乱数として出力する
        // u > θのときは0を出力する
        if update(&self.x, &self.y, &self.z, &self.w) <= self.probability.get() { 1u64 }
        else { 0u64 }
    }

    /// 確率変数のパラメータを変更する
    /// * `probability` - 尺度母数
    pub fn try_set_params(&self, probability: f64) -> Result<f64, &str> {
        if !(0f64..=1f64).contains(&probability) {
            Err("発生確率尺度母数が0より小さいか、1よりも大きいです。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.probability.set(probability);
            Ok( self.probability.get() )
        }
    }
}

#[macro_export]
/// ベルヌーイ分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let bernoulli = rand_simple::create_bernoulli!(1192u32);
/// assert_eq!(bernoulli.sample(), 0u64);
/// ```
/// # 使用例 2
/// ```
/// let bernoulli = rand_simple::create_bernoulli!();
/// println!("乱数: {}", bernoulli.sample()); // インスタンス生成時刻に依存するため、コンパイル時は値不明
/// ```
macro_rules! create_bernoulli {
    () => {
        $crate::Bernoulli::new($crate::create_seed())
    };
    ($seed: expr) => {
        $crate::Bernoulli::new($seed as u32)
    };
}


impl std::fmt::Display for Bernoulli {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "発生確率: {}", self.probability.get())?;
        Ok(())
    }
}