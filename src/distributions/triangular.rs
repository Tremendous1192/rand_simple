use crate::standard_distributions::xorshift160_0_1;
use crate::{create_state, Triangular};

impl Triangular {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            min: 0_f64,
            max: 1_f64,
            mode: 0.5_f64,
        }
    }

    /// 一様分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.95 (逆関数法)
        let s = self.max - self.min;
        let d = (self.mode - self.min) / s;
        let d_1m = 1_f64 - d;

        // step 1
        let u = xorshift160_0_1(&mut self.xyzuv);

        // step 2
        let y = if u < d {
            (d * u).sqrt()
        } else {
            1_f64 - (d_1m * (1_f64 - u)).sqrt()
        };

        // step 3
        self.min + s * y
    }

    /// 確率変数のパラメータを変更する
    /// * `min` - 最小値
    /// * `max` - 最大値
    /// * `mode` - モード
    pub fn try_set_params(
        &mut self,
        min: f64,
        max: f64,
        mode: f64,
    ) -> Result<(f64, f64, f64), &str> {
        if min >= max {
            Err("最小値と最大値が等しい、あるいは最小値の方が大きいです。確率変数のパラメータは前回の設定を維持します。")
        } else if mode < min || max < mode {
            Err("モードが最小値よりも小さい、あるいはモードが最大値よりも大きいです。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.min = min;
            self.max = max;
            self.mode = mode;
            Ok((self.min, self.max, self.mode))
        }
    }
}

impl std::fmt::Display for Triangular {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 範囲(閉区間)
    /// * モード
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "閉区間: [{}, {}]", self.min, self.max)?;
        writeln!(f, "モード: {}", self.mode)?;
        Ok(())
    }
}
