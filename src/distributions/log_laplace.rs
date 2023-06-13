use crate::{LogLaplace, create_state};
use crate::standard_distributions::{xorshift160_0_1, standard_laplace};

impl LogLaplace {
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
            location: 0f64,
            scale: 1f64,
        }
    }

    /// 対数ラプラス分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        (standard_laplace(&mut self.x, &mut self.y, &mut self.z, &mut self.u, &mut self.v, &mut self.previous_uniform_1) * self.scale + self.location).exp()     
    }

    /// 確率変数のパラメータを変更する
    /// * `location` - 位置母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, location: f64, scale: f64) -> Result<(f64, f64), &str> {
        if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.location = location;
            self.scale = scale;
            Ok( (location, scale) )
        }
    }
}


#[macro_export]
/// 対数ラプラス分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut log_laplace = rand_simple::create_laplace!(1192u32);
/// println!("位置母数 μ = 0, 尺度母数 θ = 1 の標準対数ラプラス分布に従う乱数を生成する -> {}", log_laplace.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut log_laplace = rand_simple::create_laplace!();
/// println!("位置母数 μ = 0, 尺度母数 θ = 1 の標準対数ラプラス分布に従う乱数を生成する -> {}", log_laplace.sample());
/// ```
macro_rules! create_log_laplace {
    // 引数無し
    () => {{
        $crate::LogLaplace::new($crate::create_seed())
    }};
    // 引数有り
    ($seed: expr) => {
        $crate::LogLaplace::new($seed as u32)
    };
}


impl std::fmt::Display for LogLaplace {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 位置母数
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "位置母数: {}", self.location)?;
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}