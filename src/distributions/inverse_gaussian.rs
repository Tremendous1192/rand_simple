use crate::standard_distributions::{standard_normal, xorshift160_0_1};
use crate::{create_state, InverseGaussian};

impl InverseGaussian {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(seeds: [u32; 3]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv_u: create_state(adjusted_seeds[0]),
            xyzuv_hn_0: create_state(adjusted_seeds[1]),
            xyzuv_hn_1: create_state(adjusted_seeds[2]),
            mean: 1_f64,
            shape: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.94
        // 前処理
        let p = self.mean.powi(2);
        let q = p / (2_f64 * self.shape);

        // step 1
        let z = standard_normal(&mut self.xyzuv_hn_0, &mut self.xyzuv_hn_1).abs();
        if z == 0_f64 {
            // step 1 -> step 5
            self.mean
        } else {
            // step 2
            let v = self.mean + q * z.powi(2);
            let x_1 = v + (v.powi(2) - p).sqrt();

            // step 3
            let u = xorshift160_0_1(&mut self.xyzuv_u);
            if u * (x_1 + self.mean) <= self.mean {
                // step 3 -> step 5
                x_1
            } else {
                // step 4 -> step 5
                p / x_1
            }
        }
    }

    /// 確率変数のパラメータを変更する
    /// * `mean` - 平均
    /// * `shape` - 形状母数
    pub fn try_set_params(&mut self, mean: f64, shape: f64) -> Result<(f64, f64), &str> {
        if mean <= 0_f64 {
            Err("平均が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape <= 0_f64 {
            Err("形状母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.mean = mean;
            self.shape = shape;
            core::result::Result::Ok((mean, shape))
        }
    }
}

impl std::fmt::Display for InverseGaussian {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 確率パラメータ
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "平均: {}", self.mean)?;
        writeln!(f, "形状母数: {}", self.shape)?;
        Ok(())
    }
}
