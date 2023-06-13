use crate::{ReflectedWeibull, create_state};
use crate::standard_distributions::{xorshift160_0_1, standard_laplace};

impl ReflectedWeibull {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let mut xyzuv: (u32, u32, u32, u32, u32) = create_state(_seed);
        let mut u_1: f64 = xorshift160_0_1(&mut xyzuv.0, &mut xyzuv.1, &mut xyzuv.2, &mut xyzuv.3, &mut xyzuv.4);
        while u_1 == 1f64 {
            u_1 = xorshift160_0_1(&mut xyzuv.0, &mut xyzuv.1, &mut xyzuv.2, &mut xyzuv.3, &mut xyzuv.4);
        }
        Self {
            x: xyzuv.0, y: xyzuv.1, z: xyzuv.2, u: xyzuv.3, v: xyzuv.4,
            previous_uniform_1: u_1,
            shape_inv: 1f64,
            location: 0f64,
            scale: 1f64,
        }
    }

    /// ワイブル分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        let z: f64 = standard_laplace(&mut self.x, &mut self.y, &mut self.z, &mut self.u, &mut self.v, &mut self.previous_uniform_1);
        if z >= 0f64 {
            return z.powf(self.shape_inv) * self.scale + self. location;
        }
        else {
            return (-z).powf(self.shape_inv) * self.scale + self. location;
        }
    }

    /// 確率変数のパラメータを変更する
    /// * `shape` - 形状母数
    /// * `location` - 位置母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, shape: f64, location: f64, scale: f64) -> Result<(f64, f64, f64), &str> {
        if shape <= 0f64 || scale <= 0f64 {
            Err("形状母数あるいは尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.shape_inv = shape.powi(-1);
            self.location = location;
            self.scale = scale;
            Ok( (shape, location, scale) )
        }
    }
}


#[macro_export]
/// 反射ワイブル分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut reflected_weibull = rand_simple::create_reflected_weibull!(1192u32);
/// println!("形状母数 γ = 1, 位置母数 μ = 0, 尺度母数 η = 1 の反射ワイブル分布に従う乱数を生成する -> {}", reflected_weibull.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut reflected_weibull = rand_simple::create_reflected_weibull!();
/// println!("形状母数 γ = 1, 位置母数 μ = 0, 尺度母数 η = 1 の反射ワイブル分布に従う乱数を生成する -> {}", reflected_weibull.sample());
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
        writeln!(f, "形状母数: {}", self.shape_inv.powi(-1))?;
        writeln!(f, "位置母数: {}", self.location)?;
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}