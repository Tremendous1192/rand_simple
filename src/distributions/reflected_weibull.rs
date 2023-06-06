use crate::{ReflectedWeibull, initialize, update};
use std::cell::Cell;

impl ReflectedWeibull {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzw: (u32, u32, u32, u32) = initialize(_seed);
        Self {
            x: Cell::new(xyzw.0), y: Cell::new(xyzw.1), z: Cell::new(xyzw.2), w: Cell::new(xyzw.3),
            shape_inv: Cell::new(1f64),
            location: Cell::new(0f64),
            scale: Cell::new(1f64),
        }
    }

    /// ワイブル分布に従う乱数を返す
    pub fn sample(&self) -> f64 {
        // アルゴリズム 3.53
        // step 1: 区間(0, 1) かつ 1/2 ではない一様乱数Uを生成する
        let mut u = update(&self.x, &self.y, &self.z, &self.w);
        while u == 1f64 || u == 0f64 || u == 0.5f64 {u = update(&self.x, &self.y, &self.z, &self.w);}

        // step 2: U < 1/2 のとき、Y = - (-ln2U)^γ_inve を、
        //         U > 1/2 のとき、Y = - (-ln2(1 -U))^γ_inve を生成する
        let y = if u < 0.5f64 { -(-(2f64 * u).ln()).powf(self.shape_inv.get()) }
        else { (-(2f64 * (1f64 - u)).ln()).powf(self.shape_inv.get()) };
        // step 3: X = μ + ηY を所望の乱数として生成する
        self.location.get() + self.scale.get() * y
    }

    /// 確率変数のパラメータを変更する
    /// * `shape` - 形状母数
    /// * `location` - 位置母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&self, shape: f64, location: f64, scale: f64) -> Result<(f64, f64, f64), &str> {
        if shape <= 0f64 || scale <= 0f64 {
            Err("形状母数あるいは尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.shape_inv.set(shape.powi(-1));
            self.location.set(location);
            self.scale.set(scale);
            Ok( (self.shape_inv.get().powi(-1), self.location.get(), self.scale.get()) )
        }
    }
}


#[macro_export]
/// 反射ワイブル分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let weibull = rand_simple::create_reflected_weibull!(1192u32);
/// assert_eq!(weibull.sample(), 1.2122183368953001f64);
/// ```
/// # 使用例 2
/// ```/// let weibull = rand_simple::create_reflected_weibull!();
/// println!("乱数: {}", weibull.sample()); // インスタンス生成時刻に依存するため、コンパイル時は値不明
/// ```
macro_rules! create_reflected_weibull {
    () => {{
        $crate::ReflectedWeibull::new($crate::create_seed())
    }};
    ($seed: expr) => {
        $crate::ReflectedWeibull::new($seed as u32)
    };
}


impl std::fmt::Display for ReflectedWeibull {
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