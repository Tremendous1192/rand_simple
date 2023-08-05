use crate::standard_distributions::xorshift160_0_1_open;
use crate::{create_state, PowerFunction};

impl PowerFunction {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(seed: u32) -> Self {
        let xyzuv: [u32; 5] = create_state(seed);
        Self {
            xyzuv,
            shape_gamma: 1_f64,
            min_a: 0_f64,
            max_b: 1_f64,
        }
    }

    /// べき関数分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム3.67: 逆関数法
        xorshift160_0_1_open(&mut self.xyzuv).powf(-self.shape_gamma) * (self.max_b - self.min_a)
            + self.min_a
    }

    /// 確率変数のパラメータを変更する
    /// * `shape_gamma` - 形状母数 γ
    /// * `min_a` - 境界母数の小範 a
    /// * `max_b` - 境界母数の大範 b
    pub fn try_set_params(
        &mut self,
        shape_gamma: f64,
        min_a: f64,
        max_b: f64,
    ) -> Result<(f64, f64, f64), &str> {
        if shape_gamma <= 0f64 {
            Err("形状母数 γ が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if min_a >= max_b {
            Err("境界母数の大範 b が小範 a と等しいあるいは大きいです。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape_gamma = shape_gamma;
            self.min_a = min_a;
            self.max_b = max_b;
            Ok((shape_gamma, min_a, max_b))
        }
    }
}

impl std::fmt::Display for PowerFunction {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "形状母数 γ: {}", self.shape_gamma)?;
        writeln!(f, "境界母数(小範) a: {}", self.min_a)?;
        writeln!(f, "境界母数(大範) b: {}", self.max_b)?;
        Ok(())
    }
}
