use crate::standard_distributions::xorshift160_0_1;
use crate::{create_state, Uniform};

impl Uniform {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            min: 0_f64,
            range: 1_f64,
        }
    }

    /// 一様分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        xorshift160_0_1(&mut self.xyzuv) * self.range + self.min
    }

    /// 確率変数のパラメータを変更する
    /// * `min` - 最小値
    /// * `max` - 最大値
    pub fn try_set_params(&mut self, min: f64, max: f64) -> Result<(f64, f64), &str> {
        if min >= max {
            Err("最小値と最大値が等しい、あるいは最小値の方が大きいです。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.min = min;
            self.range = max - min;
            Ok((self.min, self.min + self.range))
        }
    }
}

impl std::fmt::Display for Uniform {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 範囲(閉区間)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "閉区間: [{}, {}]", self.min, (self.min + self.range))?;
        Ok(())
    }
}
