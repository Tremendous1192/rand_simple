use crate::standard_distributions::standard_gamma;
use crate::{create_state, Gamma};

impl Gamma {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(seeds: [u32; 3]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv: create_state(adjusted_seeds[0]),
            xyzuv0: create_state(adjusted_seeds[1]),
            xyzuv1: create_state(adjusted_seeds[2]),
            shape: 1f64,
            scale: 1f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        standard_gamma(
            &mut self.xyzuv,
            &mut self.xyzuv0,
            &mut self.xyzuv1,
            &self.shape,
        ) * self.scale
    }

    /// 確率変数のパラメータを変更する
    /// * `shape` - 形状母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, shape: f64, scale: f64) -> Result<(f64, f64), &str> {
        if shape <= 0_f64 {
            Err("形状母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape == 1_f64 / 3_f64 {
            Err("形状母数が1/3です。確率変数のパラメータは前回の設定を維持します。")
        } else if scale <= 0_f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape = shape;
            self.scale = scale;
            Ok((shape, scale))
        }
    }
}

impl core::fmt::Display for Gamma {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        core::writeln!(f, "形状母数: {}", self.shape)?;
        core::writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}
