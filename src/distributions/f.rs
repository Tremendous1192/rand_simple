use crate::standard_distributions::{standard_gamma, xorshift160_0_1_open};
use crate::{create_state, FDistribution};

impl FDistribution {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(seeds: [u32; 8_usize]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);

        Self {
            xyzuv_alpha_1: create_state(adjusted_seeds[0]),
            xyzuv0_alpha_1: create_state(adjusted_seeds[1]),
            xyzuv1_alpha_1: create_state(adjusted_seeds[2]),
            shape_alpha_1: 1_f64,

            xyzuv_beta_1: create_state(adjusted_seeds[3]),

            degree_of_freedom_1: 1_f64,

            xyzuv_alpha_2: create_state(adjusted_seeds[4]),
            xyzuv0_alpha_2: create_state(adjusted_seeds[5]),
            xyzuv1_alpha_2: create_state(adjusted_seeds[6]),
            shape_alpha_2: 1_f64,

            xyzuv_beta_2: create_state(adjusted_seeds[7]),

            degree_of_freedom_2: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.83
        // step 1: χ二乗分布の確率変数を2個生成する
        let chi_1 = if self.degree_of_freedom_1 > 1_f64 {
            standard_gamma(
                &mut self.xyzuv_alpha_1,
                &mut self.xyzuv0_alpha_1,
                &mut self.xyzuv1_alpha_1,
                &self.shape_alpha_1,
            ) * 2_f64
        } else {
            let y = standard_gamma(
                &mut self.xyzuv_alpha_1,
                &mut self.xyzuv0_alpha_1,
                &mut self.xyzuv1_alpha_1,
                &(3_f64 / 2_f64),
            ) * 2_f64;
            let u = xorshift160_0_1_open(&mut self.xyzuv_beta_1);
            u.powi(2) * y * 2_f64
        };

        let chi_2 = if self.degree_of_freedom_2 > 1_f64 {
            standard_gamma(
                &mut self.xyzuv_alpha_2,
                &mut self.xyzuv0_alpha_2,
                &mut self.xyzuv1_alpha_2,
                &self.shape_alpha_2,
            ) * 2_f64
        } else {
            let y = standard_gamma(
                &mut self.xyzuv_alpha_2,
                &mut self.xyzuv0_alpha_2,
                &mut self.xyzuv1_alpha_2,
                &(3_f64 / 2_f64),
            ) * 2_f64;
            let u = xorshift160_0_1_open(&mut self.xyzuv_beta_2);
            u.powi(2) * y * 2_f64
        };

        // step 2: 確率変数を計算する
        self.degree_of_freedom_1 * chi_1 / chi_2 / self.degree_of_freedom_2
    }

    /// 確率変数のパラメータを変更する
    /// * `degree_of_freedom` - 自由度 r
    pub fn try_set_params(
        &mut self,
        degree_of_freedom_1: u64,
        degree_of_freedom_2: u64,
    ) -> Result<(u64, u64), &str> {
        if degree_of_freedom_1 < 1_u64 || degree_of_freedom_1 < 2_u64 {
            Err("自由度は自然数である必要があります。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.degree_of_freedom_1 = degree_of_freedom_1 as f64;
            self.degree_of_freedom_2 = degree_of_freedom_2 as f64;
            Ok((degree_of_freedom_1, degree_of_freedom_2))
        }
    }
}

impl std::fmt::Display for FDistribution {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "自由度 1: {}", self.degree_of_freedom_1 as u64)?;
        writeln!(f, "自由度 2: {}", self.degree_of_freedom_2 as u64)?;
        Ok(())
    }
}
