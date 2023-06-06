use crate::{Rayleigh, initialize, update};
use std::cell::Cell;

impl Rayleigh {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzw: (u32, u32, u32, u32) = initialize(_seed);
        Self {
            x: Cell::new(xyzw.0), y: Cell::new(xyzw.1), z: Cell::new(xyzw.2), w: Cell::new(xyzw.3),
            scale: Cell::new(1f64),
        }
    }

    /// 指数分布に従う乱数を返す
    pub fn sample(&self) -> f64 {
        // アルゴリズム 3.51
        // 標準指数分布に従う乱数z≧0を生成する
        loop {
            // Exp step 1: [0, 1)の一様乱数を生成する
            let u = update(&self.x, &self.y, &self.z, &self.w);
            if u < 1f64 {
                let mut u_dash: f64 = 1f64 - u;

                // Exp step 2:
                let mut a: f64 =0f64;

                loop {
                    // Exp step 3: u" = 2u'
                    let u_dash_dash = 2f64 * u_dash;

                    // Exp step 4
                    if u_dash_dash < 1f64 {
                        a += std::f64::consts::LN_2;
                        u_dash = u_dash_dash;
                    }
                    else {
                        // Exp step 5
                        let z = a + std::f64::consts::LN_2 * (u_dash_dash - 1f64);

                        // step 1: 標準指数分布に従う乱数z≧0を生成する
                        if z < 0f64 { break; }
                        // step 2: 戻り値を計算する
                        return (2f64 * z).sqrt() * self.scale.get();
                    }
                }
            }
        }        
    }

    /// 確率変数のパラメータを変更する
    /// * `scale` - 尺度母数
    pub fn try_set_params(&self, scale: f64) -> Result<f64, &str> {
        if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.scale.set(scale);
            Ok( self.scale.get() )
        }
    }
}


#[macro_export]
/// レイリー分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let rayleigh = rand_simple::create_rayleigh!(1192u32);
/// assert_eq!(rayleigh.sample(), 1.742465812716269f64);
/// ```
/// # 使用例 2
/// ```
/// let rayleigh = rand_simple::create_rayleigh!();
/// println!("乱数: {}", rayleigh.sample()); // インスタンス生成時刻に依存するため、コンパイル時は値不明
/// ```
macro_rules! create_rayleigh {
    () => {{
        $crate::Rayleigh::new($crate::create_seed())
    }};
    ($seed: expr) => {
        $crate::Rayleigh::new($seed as u32)
    };
}


impl std::fmt::Display for Rayleigh {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "尺度母数: {}", self.scale.get())?;
        Ok(())
    }
}