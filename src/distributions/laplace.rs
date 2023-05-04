use crate::{Laplace, set_state, update_and_uniform};

impl Laplace {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzw: set_state(_seed),
        }
    }

    /// 位置母数 μ = 0, 尺度母数 θ = 1の乱数を返す
    pub fn sample(&self) -> f64 {
        loop {
            // step 1: [0, 1)の一様乱数を生成する
            let u = update_and_uniform(&self.xyzw);
            if 0f64 < u {
                let u_dash: f64 = 2f64 * u;

                // step 2:
                let sign = if u_dash < 1f64 { 1f64 } else { -1f64 };
                let mut u_dash_dash = if u_dash < 1f64 { 1f64 - u_dash } else { 2f64 - u_dash };

                // step 3:
                let mut a: f64 =0f64;

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
                        return sign * (a + std::f64::consts::LN_2 * u);
                    }
                }
            }
        }        
    }
}


#[macro_export]
/// 一様分布のインスタンスを生成するマクロ
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