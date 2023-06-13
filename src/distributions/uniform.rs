use crate::{Uniform, create_state};
use crate::standard_distributions::xorshift160_0_1;

impl Uniform {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzuv: (u32, u32, u32, u32, u32) = create_state(_seed);
        Self {
            x: xyzuv.0, y: xyzuv.1, z: xyzuv.2, u: xyzuv.3, v: xyzuv.4,
            min: 0f64,
            range: 1f64,
        }
    }

    /// 一様分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        xorshift160_0_1(&mut self.x, &mut self.y, &mut self.z, &mut self.u, &mut self.v) * self.range + self.min
    }

    /// 確率変数のパラメータを変更する
    /// * `min` - 最小値
    /// * `max` - 最大値
    pub fn try_set_params(&mut self, min: f64, max: f64) -> Result<(f64, f64), &str> {
        if min >= max {
            Err("最小値と最大値が等しい、あるいは最小値の方が大きいです。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.min = min;
            self.range = max - min;
            Ok( (self.min, self.min + self.range) )
        }
    }
}


#[macro_export]
/// 一様分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut uniform = rand_simple::create_uniform!(1192u32);
/// assert_eq!(uniform.sample(), 0.66687147451259f64);
/// ```
/// # 使用例 2
/// ```
/// let mut uniform = rand_simple::create_uniform!();
/// println!("乱数: {}", uniform.sample()); // インスタンス生成時刻に依存するため、コンパイル時は値不明
/// ```
macro_rules! create_uniform {
    () => {{
        $crate::Uniform::new($crate::create_seed())
    }};
    ($seed: expr) => {
        $crate::Uniform::new($seed as u32)
    };
}


impl std::fmt::Display for Uniform {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 範囲(閉区間)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "範囲: [{}, {}]", self.min, (self.min + self.range))?;
        Ok(())
    }
}