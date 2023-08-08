use crate::standard_distributions::standard_normal;
use crate::{create_state, HalfNormal};

impl HalfNormal {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(seeds: [u32; 2]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv0: create_state(adjusted_seeds[0]),
            xyzuv1: create_state(adjusted_seeds[1]),
            std: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        standard_normal(&mut self.xyzuv0, &mut self.xyzuv1).abs() * self.std
    }

    /// 確率変数のパラメータを変更する
    /// * `variance` - 分散
    pub fn try_set_params(&mut self, variance: f64) -> Result<f64, &str> {
        if variance <= 0_f64 {
            core::result::Result::Err("分散が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.std = variance.sqrt();
            core::result::Result::Ok(variance)
        }
    }
}

impl core::fmt::Display for HalfNormal {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 分散
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        core::writeln!(f, "分散: {}", self.std.powi(2))?;
        core::result::Result::Ok(())
    }
}
