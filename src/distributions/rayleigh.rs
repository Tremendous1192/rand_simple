use crate::standard_distributions::standard_exponential;
use crate::{create_state, Rayleigh};
use core::result::Result::{Ok, Err};

impl Rayleigh {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            scale: 1f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.51
        (2f64 * standard_exponential(&mut self.xyzuv)).sqrt() * self.scale
    }

    /// 確率変数のパラメータを変更する
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, scale: f64) -> Result<f64, &str> {
        if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.scale = scale;
            Ok(self.scale)
        }
    }
}

impl core::fmt::Display for Rayleigh {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        core::writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}
