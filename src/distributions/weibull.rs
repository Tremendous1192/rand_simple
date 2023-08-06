use crate::standard_distributions::{standard_exponential, xorshift160_0_1_open};
use crate::{create_state, Weibull};

impl Weibull {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            shape_inv: 1_f64,
            scale: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        loop {
            let z = standard_exponential(&mut self.xyzuv);
            if z > 0_f64 {
                return z.powf(self.shape_inv) * self.scale;
            }
        }
    }

    /// 確率変数のパラメータを変更する
    /// * `shape` - 形状母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, shape: f64, scale: f64) -> Result<(f64, f64), &str> {
        if shape <= 0_f64 || scale <= 0_f64 {
            Err("形状母数あるいは尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape_inv = shape.powi(-1);
            self.scale = scale;
            Ok((shape, scale))
        }
    }
}

impl std::fmt::Display for Weibull {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 形状母数
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "形状母数: {}", self.shape_inv.powi(-1))?;
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}
