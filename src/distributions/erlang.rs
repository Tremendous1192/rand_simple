use crate::standard_distributions::standard_gamma;
use crate::{create_state, Erlang};

impl Erlang {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(seeds: [u32; 3_usize]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv_u: create_state(adjusted_seeds[0]),
            xyzuv_n_0: create_state(adjusted_seeds[1]),
            xyzuv_n_1: create_state(adjusted_seeds[2]),
            shape: 1_f64,
            scale: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        standard_gamma(
            &mut self.xyzuv_u,
            &mut self.xyzuv_n_0,
            &mut self.xyzuv_n_1,
            &self.shape,
        ) * self.scale
    }

    /// 確率変数のパラメータを変更する
    /// * `shape` - 形状母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, shape: i64, scale: f64) -> Result<(i64, f64), &str> {
        if shape <= 0_i64 {
            Err("形状母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape as f64 == 1_f64 / 3_f64 {
            Err("形状母数が1/3です。確率変数のパラメータは前回の設定を維持します。")
        } else if scale <= 0_f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape = shape as f64;
            self.scale = scale;
            Ok((shape, scale))
        }
    }
}

impl std::fmt::Display for Erlang {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        writeln!(f, "形状母数: {}", self.shape)?;
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}
