use crate::standard_distributions::xorshift160_0_open_1_open;
use crate::{create_state, PowerFunction};
use core::result::Result::{Ok, Err};

impl PowerFunction {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            shape_inv: 1_f64,
            min_a: 0_f64,
            range_s: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.67
        // step 1
        // step 2
        xorshift160_0_open_1_open(&mut self.xyzuv).powf(self.shape_inv) * self.range_s + self.min_a
    }

    /// 確率変数のパラメータを変更する
    /// * `shape` - 形状母数 γ
    /// * `min_a` - 境界母数の小範 a
    /// * `max_b` - 境界母数の大範 b
    pub fn try_set_params(
        &mut self,
        shape: f64,
        min_a: f64,
        max_b: f64,
    ) -> Result<(f64, f64, f64), &str> {
        if shape <= 0f64 {
            Err("形状母数 γ が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if min_a >= max_b {
            Err("境界母数の大範 b が小範 a と等しいあるいは大きいです。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape_inv = shape.powi(-1);
            self.min_a = min_a;
            self.range_s = max_b - min_a;
            Ok((shape, min_a, max_b))
        }
    }
}

impl core::fmt::Display for PowerFunction {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        writeln!(f, "形状母数 γ: {}", self.shape_inv.powi(-1))?;
        writeln!(f, "境界母数(小範) a: {}", self.min_a)?;
        writeln!(f, "境界母数(大範) b: {}", self.range_s + self.min_a)?;
        Ok(())
    }
}
