use crate::standard_distributions::{standard_laplace, xorshift160_0_open_1_open};
use crate::{create_state, Laplace};

impl Laplace {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        //let mut xyzuv: [u32; 5] = create_state(_seed);
        //let u_1: f64 = xorshift160_0_1_open(&mut xyzuv);
        Self {
            xyzuv: create_state(_seed),
            //previous_uniform_1: u_1,
            location: 0f64,
            scale: 1f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.45
        let u: f64 = xorshift160_0_open_1_open(&mut self.xyzuv);
        let y = if u < 0.5_f64 {
            (2_f64 * u).ln()
        } else {
            -(2_f64 * (1_f64 - u)).ln()
        };
        y * self.scale + self.location
    }

    /// 確率変数のパラメータを変更する
    /// * `location` - 位置母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, location: f64, scale: f64) -> Result<(f64, f64), &str> {
        if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.location = location;
            self.scale = scale;
            Ok((location, scale))
        }
    }
}

impl std::fmt::Display for Laplace {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 位置母数
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "位置母数: {}", self.location)?;
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}
