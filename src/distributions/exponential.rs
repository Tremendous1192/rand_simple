use crate::{Exponential, create_state};
use crate::standard_distributions::{xorshift160_0_1_open, standard_exponential};

impl Exponential {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let mut xyzuv: [u32; 5] = create_state(_seed);
        let u_1: f64 = xorshift160_0_1_open(&mut xyzuv);
        Self {
            xyzuv,
            previous_uniform_1: u_1,
            scale: 1f64,
        }
    }

    /// 指数分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        standard_exponential(&mut self.xyzuv, &mut self.previous_uniform_1) * self.scale
    }

    /// 確率変数のパラメータを変更する
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, scale: f64) -> Result<f64, &str> {
        if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.scale = scale;
            Ok( scale )
        }
    }
}


#[macro_export]
/// 指数分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut exponential = rand_simple::create_exponential!(1192u32);
/// println!("尺度母数 θ = 1の標準指数分布に従う乱数を生成する -> {}", exponential.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut exponential = rand_simple::create_exponential!();
/// println!("尺度母数 θ = 1の標準指数分布に従う乱数を生成する -> {}", exponential.sample());
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
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}