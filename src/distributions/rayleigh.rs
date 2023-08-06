use crate::standard_distributions::{standard_exponential, xorshift160_0_1_open};
use crate::{create_state, Rayleigh};

impl Rayleigh {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        //let mut xyzuv: [u32; 5] = create_state(_seed);
        //let u_1: f64 = xorshift160_0_1_open(&mut xyzuv);
        Self {
            xyzuv: create_state(_seed),
            //previous_uniform_1: u_1,
            scale: 1f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.51
        (2f64 * (-1_f64 * (1_f64 - xorshift160_0_1_open(&mut self.xyzuv)).ln())).sqrt()
            * self.scale
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

impl std::fmt::Display for Rayleigh {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}
