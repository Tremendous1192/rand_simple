use crate::standard_distributions::standard_normal;
use crate::{create_state, Levy};

impl Levy {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(seeds: [u32; 2_usize]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv0: create_state(adjusted_seeds[0]),
            xyzuv1: create_state(adjusted_seeds[1]),
            location: 0_f64,
            scale: 1_f64,
        }
    }

    /// 乱数を計算する
    pub fn sample(&mut self) -> f64 {
        loop {
            let z = standard_normal(&mut self.xyzuv0, &mut self.xyzuv1).abs();
            if z > 0_f64 {
                return z.powi(-2_i32) * self.scale + self.location;
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

impl core::fmt::Display for Levy {
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
