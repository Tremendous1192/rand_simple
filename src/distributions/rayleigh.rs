use crate::{Rayleigh, create_state};
use crate::standard_distributions::{xorshift160_0_1_open, standard_exponential};

impl Rayleigh {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let mut xyzuv: (u32, u32, u32, u32, u32) = create_state(_seed);
        let u_1: f64 = xorshift160_0_1_open(&mut xyzuv.0, &mut xyzuv.1, &mut xyzuv.2, &mut xyzuv.3, &mut xyzuv.4);
        Self {
            x: xyzuv.0, y: xyzuv.1, z: xyzuv.2, u: xyzuv.3, v: xyzuv.4,
            previous_uniform_1: u_1,
            scale: 1f64,
        }
    }

    /// レイリー分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        (2f64 * standard_exponential(&mut self.x, &mut self.y, &mut self.z, &mut self.u, &mut self.v, &mut self.previous_uniform_1)).sqrt() * self.scale
    }

    /// 確率変数のパラメータを変更する
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, scale: f64) -> Result<f64, &str> {
        if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.scale = scale;
            Ok( self.scale )
        }
    }
}


#[macro_export]
/// レイリー分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut rayleigh = rand_simple::create_rayleigh!(1192u32);
/// println!("尺度母数 θ = 1の標準レイリー分布に従う乱数を生成する -> {}", rayleigh.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut rayleigh = rand_simple::create_rayleigh!();
/// println!("尺度母数 θ = 1の標準レイリー分布に従う乱数を生成する -> {}", rayleigh.sample());
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
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}