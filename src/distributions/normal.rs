use crate::standard_distributions::standard_normal;
use crate::{create_state, Normal};

impl Normal {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(seeds: [u32; 2]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv0: create_state(adjusted_seeds[0]),
            xyzuv1: create_state(adjusted_seeds[1]),
            mean: 0f64,
            std: 1f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        standard_normal(&mut self.xyzuv0, &mut self.xyzuv1) * self.std + self.mean
    }

    /// 確率変数のパラメータを変更する
    /// * `mean` - 平均
    /// * `variance` - 分散
    pub fn try_set_params(&mut self, mean: f64, variance: f64) -> Result<(f64, f64), &str> {
        if variance <= 0f64 {
            Err("分散が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.mean = mean;
            self.std = variance.sqrt();
            Ok((mean, variance))
        }
    }
}

impl std::fmt::Display for Normal {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 平均
    /// * 分散
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "平均: {}", self.mean)?;
        writeln!(f, "分散: {}", self.std.powi(2))?;
        Ok(())
    }
}
