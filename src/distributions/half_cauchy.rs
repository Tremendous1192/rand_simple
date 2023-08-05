use crate::standard_distributions::standard_cauchy;
use crate::{create_state, HalfCauchy};

impl HalfCauchy {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(seeds: [u32; 2]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv0: create_state(adjusted_seeds[0]),
            xyzuv1: create_state(adjusted_seeds[1]),
            scale: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        standard_cauchy(&mut self.xyzuv0, &mut self.xyzuv1).abs() * self.scale
    }

    /// 確率変数のパラメータを変更する
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, scale: f64) -> Result<f64, &str> {
        if scale <= 0_f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.scale = scale;
            Ok(scale)
        }
    }
}

impl std::fmt::Display for HalfCauchy {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}
