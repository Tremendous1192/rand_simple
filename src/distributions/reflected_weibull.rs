use crate::standard_distributions::xorshift160_0_open_1_open;
use crate::{create_state, ReflectedWeibull};

impl ReflectedWeibull {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            shape_inv: 1_f64,
            location: 0_f64,
            scale: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.53: 逆関数法
        // step 1: (0, 1) の一様乱数
        let u = xorshift160_0_open_1_open(&mut self.xyzuv);
        // step 2
        if u < 0.5_f64 {
            -(-(2_f64 * u).ln()).powf(self.shape_inv) * self.scale + self.location
        } else {
            (-(2_f64 * (1_f64 - u)).ln()).powf(self.shape_inv) * self.scale + self.location
        }
    }

    /// 確率変数のパラメータを変更する
    /// * `shape` - 形状母数
    /// * `location` - 位置母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(
        &mut self,
        shape: f64,
        location: f64,
        scale: f64,
    ) -> core::result::Result<(f64, f64, f64), &str> {
        if shape <= 0_f64 || scale <= 0_f64 {
            core::result::Result::Err("形状母数あるいは尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape_inv = shape.powi(-1);
            self.location = location;
            self.scale = scale;
            core::result::Result::Ok((shape, location, scale))
        }
    }
}

impl core::fmt::Display for ReflectedWeibull {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 形状母数
    /// * 尺度母数
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        core::writeln!(f, "形状母数: {}", self.shape_inv.powi(-1))?;
        core::writeln!(f, "位置母数: {}", self.location)?;
        core::writeln!(f, "尺度母数: {}", self.scale)?;
        core::result::Result::Ok(())
    }
}
