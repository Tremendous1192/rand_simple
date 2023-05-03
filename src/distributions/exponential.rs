use crate::{Exponential, set_state, update_and_uniform};

impl Exponential {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzw: set_state(_seed),
        }
    }

    /// 閉区間[0, 1]の乱数を返す
    pub fn sample(&self) -> f64 {
        loop {
            // step 1: [0, 1)の一様乱数を生成する
            let u = update_and_uniform(&self.xyzw);
            if 0f64 < u {
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
/// 一様分布のインスタンスを生成するマクロ
macro_rules! create_exponential {
    // 引数無し
    () => {{
        $crate::Exponential::new($crate::create_seed())
    }};
    // 引数有り
    ($seed: expr) => {
        $crate::Exponential::new($seed as u32)
    };
}