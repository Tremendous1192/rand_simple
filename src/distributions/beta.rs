use crate::standard_distributions::standard_gamma;
use crate::{create_state, Beta};

impl Beta {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(seeds: [u32; 6]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);

        Self {
            xyzuv_alpha: create_state(adjusted_seeds[0]),
            xyzuv0_alpha: create_state(adjusted_seeds[1]),
            xyzuv1_alpha: create_state(adjusted_seeds[2]),
            shape_alpha: 1f64,

            xyzuv_beta: create_state(adjusted_seeds[3]),
            xyzuv0_beta: create_state(adjusted_seeds[4]),
            xyzuv1_beta: create_state(adjusted_seeds[5]),
            shape_beta: 1f64,
        }
    }

    /// ベータ分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        let y1 = standard_gamma(
            &mut self.xyzuv_alpha,
            &mut self.xyzuv0_alpha,
            &mut self.xyzuv1_alpha,
            &self.shape_alpha,
        );
        let y2 = standard_gamma(
            &mut self.xyzuv_beta,
            &mut self.xyzuv0_beta,
            &mut self.xyzuv1_beta,
            &self.shape_beta,
        );
        y1 / (y1 + y2)
    }

    /// 確率変数のパラメータを変更する
    /// * `shape_alpha` - 形状母数 α
    /// * `shape_beta` - 形状母数 β
    pub fn try_set_params(
        &mut self,
        shape_alpha: f64,
        shape_beta: f64,
    ) -> Result<(f64, f64), &str> {
        if shape_alpha <= 0f64 {
            Err("形状母数 α が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape_alpha == 1f64 / 3f64 {
            Err("形状母数 α が1/3です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape_beta <= 0f64 {
            Err("形状母数 β が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape_beta == 1f64 / 3f64 {
            Err("形状母数 β が1/3です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape_alpha = shape_alpha;
            self.shape_beta = shape_beta;
            Ok((shape_alpha, shape_beta))
        }
    }
}

impl core::fmt::Display for Beta {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        core::writeln!(f, "形状母数 α: {}", self.shape_alpha)?;
        core::writeln!(f, "形状母数 β: {}", self.shape_beta)?;
        Ok(())
    }
}
