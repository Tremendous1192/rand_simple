use crate::{Uniform, initialize, update};
use std::cell::Cell;

impl Uniform {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzw: (u32, u32, u32, u32) = initialize(_seed);
        Self {
            x: Cell::new(xyzw.0), y: Cell::new(xyzw.1), z: Cell::new(xyzw.2), w: Cell::new(xyzw.3),
            min: Cell::new(0f64),
            range: Cell::new(1f64),
        }
    }

    /// 一様分布に従う乱数を返す
    pub fn sample(&self) -> f64 {
        update(&self.x, &self.y, &self.z, &self.w) * self.range.get() + self.min.get()
    }

    /// 確率変数のパラメータを変更する
    /// * `min` - 最小値
    /// * `max` - 最大値
    pub fn try_set_params(&self, min: f64, max: f64) -> Result<(f64, f64), &str> {
        if min >= max {
            Err("最小値と最大値が等しい、あるいは最小値の方が大きいです。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.min.set(min);
            self.range.set(max - min);
            Ok( (self.min.get(), self.min.get() + self.range.get()) )
        }
    }
}


#[macro_export]
/// 一様分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let uniform = rand_simple::create_uniform!(1192u32);
/// assert_eq!(uniform.sample(), 0.8512317447111084f64);
/// ```
/// # 使用例 2
/// ```
/// let uniform = rand_simple::create_uniform!();
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
        writeln!(f, "範囲: [{}, {}]", self.min.get(), (self.min.get() + self.range.get()))?;
        Ok(())
    }
}