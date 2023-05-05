use crate::{Exponential, set_state, update_and_uniform};

impl Exponential {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzw: set_state(_seed),
        }
    }

    /// 標準指数分布に従う乱数を返す
    /// * 尺度母数 1
    pub fn sample(&self) -> f64 {
        // アルゴリズム 3.42
        loop {
            // step 1: [0, 1)の一様乱数を生成する
            let u = update_and_uniform(&self.xyzw);
            if u < 1f64 {
                let mut u_dash: f64 = 1f64 - u;

                // step 2:
                let mut a: f64 =0f64;

                loop {
                    // step 3: u" = 2u'
                    let u_dash_dash = 2f64 * u_dash;

                    // step 4
                    if u_dash_dash < 1f64 {
                        a += std::f64::consts::LN_2;
                        u_dash = u_dash_dash;
                    }
                    else {
                        // step 5
                        return a + std::f64::consts::LN_2 * (u_dash_dash - 1f64);
                    }
                }
            }
        }        
    }
}


#[macro_export]
/// 指数分布のインスタンスを生成するマクロ
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
macro_rules! create_exponential {
    () => {{
        $crate::Exponential::new($crate::create_seed())
    }};
    ($seed: expr) => {
        $crate::Exponential::new($seed as u32)
    };
}