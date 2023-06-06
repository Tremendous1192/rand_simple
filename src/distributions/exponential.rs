use crate::{Exponential, initialize, update};
use std::cell::Cell;

impl Exponential {
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
        // アルゴリズム 3.41
        // step 1: U ~ [0, 1)
        let mut u: f64 = update(&self.x, &self.y, &self.z, &self.w);
        while u == 1f64 {u = update(&self.x, &self.y, &self.z, &self.w);}

        // step 2: Y = -ln(1-U)
        // step 3: X = θY
        - self.scale.get() * (1f64 - u).ln()
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
/// 指数分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let exponential = rand_simple::create_exponential!(1192u32);
/// assert_eq!(exponential.sample(), 1.9053655174552453f64);
/// ```
/// # 使用例 2
/// ```/// let exponential = rand_simple::create_exponential!();
/// println!("乱数: {}", exponential.sample()); // インスタンス生成時刻に依存するため、コンパイル時は値不明
/// ```
macro_rules! create_exponential {
    () => {{
        $crate::Exponential::new($crate::create_seed())
    }};
    ($seed: expr) => {
        $crate::Exponential::new($seed as u32)
    };
}


impl std::fmt::Display for Exponential {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "尺度母数: {}", self.scale.get())?;
        Ok(())
    }
}