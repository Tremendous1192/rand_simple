use crate::standard_distributions::{standard_gamma, xorshift160_0_1_open};
use crate::{create_state, ChiSquare};

impl ChiSquare {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(seeds: [u32; 4_usize]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);

        Self {
            xyzuv_alpha: create_state(adjusted_seeds[0]),
            xyzuv0_alpha: create_state(adjusted_seeds[1]),
            xyzuv1_alpha: create_state(adjusted_seeds[2]),
            shape_alpha: 1_f64,

            xyzuv_beta: create_state(adjusted_seeds[3]),

            degree_of_freedom: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.79
        if self.degree_of_freedom > 1_f64 {
            standard_gamma(
                &mut self.xyzuv_alpha,
                &mut self.xyzuv0_alpha,
                &mut self.xyzuv1_alpha,
                &self.shape_alpha,
            ) * 2_f64
        } else {
            let y = standard_gamma(
                &mut self.xyzuv_alpha,
                &mut self.xyzuv0_alpha,
                &mut self.xyzuv1_alpha,
                &(3_f64 / 2_f64),
            ) * 2_f64;
            let u = xorshift160_0_1_open(&mut self.xyzuv_beta);
            u.powi(2) * y * 2_f64
        }
    }

    /// 確率変数のパラメータを変更する
    /// * `degree_of_freedom` - 自由度 r
    pub fn try_set_params(&mut self, degree_of_freedom: u64) -> core::result::Result<u64, &str> {
        if degree_of_freedom < 1_u64 {
            core::result::Result::Err("自由度は自然数である必要があります。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.degree_of_freedom = degree_of_freedom as f64;
            core::result::Result::Ok(degree_of_freedom)
        }
    }
}

impl core::fmt::Display for ChiSquare {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        core::writeln!(f, "自由度: {}", self.degree_of_freedom as u64)?;
        core::result::Result::Ok(())
    }
}
