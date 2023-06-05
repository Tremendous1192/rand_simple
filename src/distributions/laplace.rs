use crate::{Laplace, initialize, update};
use std::cell::Cell;

impl Laplace {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzw: (u32, u32, u32, u32) = initialize(_seed);
        Self {
            x: Cell::new(xyzw.0), y: Cell::new(xyzw.1), z: Cell::new(xyzw.2), w: Cell::new(xyzw.3),
            location: Cell::new(0f64),
            scale: Cell::new(1f64),
        }
    }

    /// ラプラス分布に従う乱数を返す
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
                        return sign * (a + std::f64::consts::LN_2 * (u_dash_dash_dash - 1f64)) * self.scale.get() + self.location.get();
                    }
                }
            }
        }        
    }

    /// 確率変数のパラメータを変更する
    /// * `location` - 位置母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&self, location: f64, scale: f64) -> Result<(f64, f64), &str> {
        if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.location.set(location);
            self.scale.set(scale);
            Ok( (self.location.get(), self.scale.get()) )
        }
    }
}


#[macro_export]
/// ラプラス分布のインスタンスを生成するマクロ
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let laplace = rand_simple::create_laplace!(1192u32);
/// assert_eq!(laplace.sample(), -0.824946373682539f64);
/// ```
/// # 使用例 2
/// ```
/// let laplace = rand_simple::create_laplace!();
/// println!("乱数: {}", laplace.sample()); // インスタンス生成時刻に依存するため、コンパイル時は値不明
/// ```
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


impl std::fmt::Display for Laplace {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 位置母数
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "位置母数: {}", self.location.get())?;
        writeln!(f, "尺度母数: {}", self.scale.get())?;
        Ok(())
    }
}