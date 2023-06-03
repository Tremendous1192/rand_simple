use crate::{Laplace, initialize, update};
use std::cell::Cell;

impl Laplace {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzw: (u32, u32, u32, u32) = initialize(_seed);
        Self {
            x: Cell::new(xyzw.0), y: Cell::new(xyzw.1), z: Cell::new(xyzw.2), w: Cell::new(xyzw.3),
        }
    }

    /// 標準ラプラス分布に従う乱数を返す
    /// * 位置母数 0
    /// * 尺度母数 1
    pub fn sample(&self) -> f64 {
        // アルゴリズム 3.46
        loop {
            // step 1: [0, 1)の一様乱数を生成する
            let u = update(&self.x, &self.y, &self.z, &self.w);
            if u < 1f64 {
                let u_dash: f64 = 2f64 * u;

                // step 2:
                let sign = if u_dash < 1f64 { 1f64 } else { -1f64 };
                let mut u_dash_dash = if u_dash < 1f64 { 1f64 - u_dash } else { 2f64 - u_dash };

                // step 3:
                let mut a: f64 = 0f64;

                loop {
                    // step 4: u" = 2u'
                    let u_dash_dash_dash = 2f64 * u_dash_dash;

                    // step 5
                    if u_dash_dash_dash < 1f64 {
                        a += std::f64::consts::LN_2;
                        u_dash_dash = u_dash_dash_dash;
                    }
                    else {
                        // step 6
                        return sign * (a + std::f64::consts::LN_2 * (u_dash_dash_dash - 1f64));
                    }
                }
            }
        }        
    }
}


#[macro_export]
/// ラプラス分布のインスタンスを生成するマクロ
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
macro_rules! create_laplace {
    // 引数無し
    () => {{
        $crate::Laplace::new($crate::create_seed())
    }};
    // 引数有り
    ($seed: expr) => {
        $crate::Laplace::new($seed as u32)
    };
}