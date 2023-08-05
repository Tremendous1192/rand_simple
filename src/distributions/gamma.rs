use crate::standard_distributions::{standard_gamma, xorshift160_0_1_open};
use crate::{create_state, Gamma};

impl Gamma {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(seeds: [u32; 3]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        let mut xyzuv: [u32; 5] = create_state(adjusted_seeds[0]);
        let u_1: f64 = xorshift160_0_1_open(&mut xyzuv);
        Self {
            xyzuv,
            previous_uniform_1: u_1,
            xyzuv0: create_state(adjusted_seeds[1]),
            xyzuv1: create_state(adjusted_seeds[2]),
            shape: 1f64,
            scale: 1f64,
        }
    }

    /// ガンマ分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        standard_gamma(
            &mut self.xyzuv,
            &mut self.previous_uniform_1,
            &mut self.xyzuv0,
            &mut self.xyzuv1,
            &self.shape,
        ) * self.scale
    }

    /// 確率変数のパラメータを変更する
    /// * `shape` - 形状母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, shape: f64, scale: f64) -> Result<(f64, f64), &str> {
        if shape <= 0f64 {
            Err("形状母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape == 1f64 / 3f64 {
            Err("形状母数が1/3です。確率変数のパラメータは前回の設定を維持します。")
        } else if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape = shape;
            self.scale = scale;
            Ok((shape, scale))
        }
    }
}

impl std::fmt::Display for Gamma {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "形状母数: {}", self.shape)?;
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}
