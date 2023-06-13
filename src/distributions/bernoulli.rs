use crate::{Bernoulli, create_state};
use crate::standard_distributions::xorshift160_0_1;

impl Bernoulli {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzuv: (u32, u32, u32, u32, u32) = create_state(_seed);
        Self {
            x: xyzuv.0, y: xyzuv.1, z: xyzuv.2, u: xyzuv.3, v: xyzuv.4,
            probability: 0.5f64,
        }
    }

    /// ある確率の事象が生じたか(1u64)、否か(0u64)を返す
    pub fn sample(&mut self) -> u64 {
        if xorshift160_0_1(&mut self.x, &mut self.y, &mut self.z, &mut self.u, &mut self.v) <= self.probability { 1u64 }
        else { 0u64 }
    }

    /// 確率変数のパラメータを変更する
    /// * `probability` - 尺度母数
    pub fn try_set_params(&mut self, probability: f64) -> Result<f64, &str> {
        if !(0f64..=1f64).contains(&probability) {
            Err("発生確率が0より小さいか、1よりも大きいです。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.probability = probability;
            Ok( probability )
        }
    }
}

#[macro_export]
/// ベルヌーイ分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut bernoulli = rand_simple::create_bernoulli!(1192u32);
/// println!("発生確率 θ = 0.5 の事象が生じたか(1)、否か(0)の判定 -> {}", bernoulli.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut bernoulli = rand_simple::create_bernoulli!();
/// println!("発生確率 θ = 0.5 の事象が生じたか(1)、否か(0)の判定 -> {}", bernoulli.sample());
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
    /// * 発生確率
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "発生確率: {}", self.probability)?;
        Ok(())
    }
}