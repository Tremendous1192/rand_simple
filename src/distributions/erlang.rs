use crate::standard_distributions::standard_gamma;
use crate::{create_state, Erlang};

impl Erlang {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(seeds: [u32; 3_usize]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv: create_state(adjusted_seeds[0]),
            xyzuv0: create_state(adjusted_seeds[1]),
            xyzuv1: create_state(adjusted_seeds[2]),
            shape: 1_f64,
            scale: 1_f64,
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
    pub fn try_set_params(&mut self, shape: i64, scale: f64) -> core::result::Result<(i64, f64), &str> {
        if shape <= 0i64 {
            core::result::Result::Err("形状母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape as f64 == 1f64 / 3f64 {
            core::result::Result::Err("形状母数が1/3です。確率変数のパラメータは前回の設定を維持します。")
        } else if scale <= 0f64 {
            core::result::Result::Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape = shape as f64;
            self.scale = scale;
            core::result::Result::Ok((shape, scale))
        }
    }
}

impl core::fmt::Display for Erlang {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        core::writeln!(f, "形状母数: {}", self.shape)?;
        core::writeln!(f, "尺度母数: {}", self.scale)?;
        core::result::Result::Ok(())
    }
}
