use crate::standard_distributions::{
    standard_cauchy, standard_exponential, standard_gamma, standard_normal,
};
use crate::{create_state, TDistribution};

impl TDistribution {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(seeds: [u32; 5_usize]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);

        Self {
            xyzuv_n_0: create_state(adjusted_seeds[0]),
            xyzuv_n_1: create_state(adjusted_seeds[1]),

            xyzuv_u_gamma: create_state(adjusted_seeds[2]),
            xyzuv_n_0_gamma: create_state(adjusted_seeds[3]),
            xyzuv_n_1_gamma: create_state(adjusted_seeds[4]),

            degree_of_freedom: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        match self.degree_of_freedom as usize {
            1_usize => standard_cauchy(&mut self.xyzuv_u_gamma),
            2_usize => {
                // アルゴリズム 3.84 r: = 2
                // step 1
                let mut z = 0_f64;
                let mut w = 0_f64;
                while w == 0_f64 {
                    z = standard_normal(&mut self.xyzuv_n_0, &mut self.xyzuv_n_1);
                    w = standard_exponential(&mut self.xyzuv_u_gamma);
                }
                // step 2
                z / w.sqrt()
            }
            _ => {
                // アルゴリズム 3.85: r > 2
                // step 1
                let z = standard_normal(&mut self.xyzuv_n_0, &mut self.xyzuv_n_1);
                let w = standard_gamma(
                    &mut self.xyzuv_u_gamma,
                    &mut self.xyzuv_n_0_gamma,
                    &mut self.xyzuv_n_1_gamma,
                    &(self.degree_of_freedom / 2_f64),
                );
                // step 2
                self.degree_of_freedom.sqrt() * z / w.sqrt()
            }
        }
    }

    /// 確率変数のパラメータを変更する
    /// * `degree_of_freedom` - 自由度 r
    pub fn try_set_params(&mut self, degree_of_freedom: u64) -> Result<u64, &str> {
        if degree_of_freedom < 1_u64 {
            Err("自由度は自然数である必要があります。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.degree_of_freedom = degree_of_freedom as f64;
            Ok(degree_of_freedom)
        }
    }
}

impl std::fmt::Display for TDistribution {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "自由度: {}", self.degree_of_freedom as u64)?;
        Ok(())
    }
}
