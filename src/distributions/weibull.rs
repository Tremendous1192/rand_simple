use crate::{Weibull, initialize, update};
use std::cell::Cell;

impl Weibull {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzw: (u32, u32, u32, u32) = initialize(_seed);
        Self {
            x: Cell::new(xyzw.0), y: Cell::new(xyzw.1), z: Cell::new(xyzw.2), w: Cell::new(xyzw.3),
            shape_inv: Cell::new(1f64),
            scale: Cell::new(1f64),
        }
    }

    /// ワイブル分布に従う乱数を返す
    pub fn sample(&self) -> f64 {
        // アルゴリズム 3.52
        // step 1: 標準指数分布に従う乱数Zをz > 0の範囲で生成する

        // 指数分布 step 1: U ~ (0, 1) (z > 0のため開区間に修正)
        let mut u: f64 = update(&self.x, &self.y, &self.z, &self.w);
        while u == 1f64 || u == 0f64 {u = update(&self.x, &self.y, &self.z, &self.w);}
        // 指数分布 step 2: Z = -ln(1-U) (標準指数分布のため、step 2で終了)
        let z: f64 = - (1f64 - u).ln();

        // step 2: Y = Z^γ_inve
        // step 3: Z = ηY
        self.scale.get() * z.powf(self.shape_inv.get())
    }

    /// 確率変数のパラメータを変更する
    /// * `shape` - 形状母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&self, shape: f64, scale: f64) -> Result<(f64, f64), &str> {
        if shape <= 0f64 || scale <= 0f64 {
            Err("形状母数あるいは尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.shape_inv.set(shape.powi(-1));
            self.scale.set(scale);
            Ok( (self.shape_inv.get().powi(-1), self.scale.get()) )
        }
    }
}


#[macro_export]
/// ワイブル分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let weibull = rand_simple::create_weibull!(1192u32);
/// assert_eq!(weibull.sample(), 1.9053655174552453f64);
/// ```
/// # 使用例 2
/// ```/// let weibull = rand_simple::create_weibull!();
/// println!("乱数: {}", weibull.sample()); // インスタンス生成時刻に依存するため、コンパイル時は値不明
/// ```
macro_rules! create_weibull {
    () => {{
        $crate::Weibull::new($crate::create_seed())
    }};
    ($seed: expr) => {
        $crate::Weibull::new($seed as u32)
    };
}


impl std::fmt::Display for Weibull {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 形状母数
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "形状母数: {}", self.shape_inv.get())?;
        writeln!(f, "尺度母数: {}", self.scale.get())?;
        Ok(())
    }
}