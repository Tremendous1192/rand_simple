use crate::standard_distributions::xorshift160_0_1;
use crate::{create_state, Geometric};

impl Geometric {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzuv: [u32; 5] = create_state(_seed);
        Self {
            xyzuv,
            probability: 0.5_f64,
        }
    }

    /// ある確率の事象が初めて生じるまでの試行回数を返す
    pub fn sample(&mut self) -> u64 {
        let mut x: u64 = 1_u64;
        while xorshift160_0_1(&mut self.xyzuv) > self.probability {
            x += 1_u64;
        }
        x
    }

    /// 確率変数のパラメータを変更する
    /// * `probability` - 発生確率
    pub fn try_set_params(&mut self, probability: f64) -> Result<f64, &str> {
        if !(0_f64..=1_f64).contains(&probability) {
            core::result::Result::Err("発生確率が0より小さいか、1よりも大きいです。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.probability = probability;
            core::result::Result::Ok(probability)
        }
    }
}

impl core::fmt::Display for Geometric {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 発生確率
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        core::writeln!(f, "発生確率: {}", self.probability)?;
        core::result::Result::Ok(())
    }
}
