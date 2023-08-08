use crate::standard_distributions::standard_exponential;
use crate::{create_state, Gunbel};

impl Gunbel {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            location: 0_f64,
            scale: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        loop {
            let z = standard_exponential(&mut self.xyzuv);
            if z > 0_f64 {
                return -z.ln() * self.scale + self.location;
            }
        }
    }

    /// 確率変数のパラメータを変更する
    /// * `location` - 位置母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, location: f64, scale: f64) -> core::result::Result<(f64, f64), &str> {
        if scale <= 0_f64 {
            core::result::Result::Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.location = location;
            self.scale = scale;
            core::result::Result::Ok((location, scale))
        }
    }
}

impl core::fmt::Display for Gunbel {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 形状母数
    /// * 尺度母数
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::writeln!(f, "構造体の型: {}", core::any::type_name::<Self>())?;
        core::writeln!(f, "位置母数: {}", self.location)?;
        core::writeln!(f, "尺度母数: {}", self.scale)?;
        core::result::Result::Ok(())
    }
}
