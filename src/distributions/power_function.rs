use crate::standard_distributions::{standard_exponential, standard_gamma};
use crate::{create_state, PowerFunction};

impl PowerFunction {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(seeds: [u32; 4]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);

        Self {
            xyzuv_alpha: create_state(adjusted_seeds[0]),
            xyzuv0_alpha: create_state(adjusted_seeds[1]),
            xyzuv1_alpha: create_state(adjusted_seeds[2]),
            shape_alpha: 1f64,

            xyzuv_beta: create_state(adjusted_seeds[3]),

            shape_gamma: 1_f64,
            min_a: 0_f64,
            range_s: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.68:
        let y1 = standard_gamma(
            &mut self.xyzuv_alpha,
            &mut self.xyzuv0_alpha,
            &mut self.xyzuv1_alpha,
            &self.shape_alpha,
        );
        let y2 = standard_exponential(&mut self.xyzuv_beta);
        y1 / (y1 * y2) * self.range_s + self.min_a
    }

    /// 確率変数のパラメータを変更する
    /// * `shape_gamma` - 形状母数 γ
    /// * `min_a` - 境界母数の小範 a
    /// * `max_b` - 境界母数の大範 b
    pub fn try_set_params(
        &mut self,
        shape_gamma: f64,
        min_a: f64,
        max_b: f64,
    ) -> Result<(f64, f64, f64), &str> {
        if shape_gamma <= 0f64 {
            Err("形状母数 γ が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if min_a >= max_b {
            Err("境界母数の大範 b が小範 a と等しいあるいは大きいです。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape_gamma = shape_gamma;
            self.min_a = min_a;
            self.range_s = max_b - min_a;
            Ok((shape_gamma, min_a, max_b))
        }
    }
}

impl std::fmt::Display for PowerFunction {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "形状母数 γ: {}", self.shape_gamma)?;
        writeln!(f, "境界母数(小範) a: {}", self.min_a)?;
        writeln!(f, "境界母数(大範) b: {}", self.range_s + self.min_a)?;
        Ok(())
    }
}
