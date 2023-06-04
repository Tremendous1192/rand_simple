use crate::{Uniform, initialize, update};
use std::cell::Cell;

impl Uniform {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzw: (u32, u32, u32, u32) = initialize(_seed);
        Self {
            x: Cell::new(xyzw.0), y: Cell::new(xyzw.1), z: Cell::new(xyzw.2), w: Cell::new(xyzw.3),
        }
    }

    /// 閉区間[0, 1]の乱数を返す
    pub fn sample(&self) -> f64 {
        update(&self.x, &self.y, &self.z, &self.w)
    }
}


#[macro_export]
/// 一様分布のインスタンスを生成するマクロ
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
macro_rules! create_uniform {
    () => {{
        $crate::Uniform::new($crate::create_seed())
    }};
    ($seed: expr) => {
        $crate::Uniform::new($seed as u32)
    };
}