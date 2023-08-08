use crate::standard_distributions::standard_cauchy;
use crate::{create_state, Cauchy};

impl Cauchy {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            location: 0_f64,
            scale: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        standard_cauchy(&mut self.xyzuv) * self.scale + self.location
    }

    /// 確率変数のパラメータを変更する
    /// * `location` - 位置母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, location: f64, scale: f64) -> Result<(f64, f64), &str> {
        if scale <= 0_f64 {
            core::result::Result::Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.location = location;
            self.scale = scale;
            core::result::Result::Ok((location, scale))
        }
    }
}

impl core::fmt::Display for Cauchy {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 位置母数
    /// * 尺度母数
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        core::writeln!(f, "位置母数: {}", self.location)?;
        core::writeln!(f, "尺度母数: {}", self.scale)?;
        core::result::Result::Ok(())
    }
}
