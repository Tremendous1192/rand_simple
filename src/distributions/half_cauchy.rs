use crate::{HalfCauchy, create_state};
use crate::standard_distributions::standard_cauchy;

impl HalfCauchy {
    /// コンストラクタ
    /// * `_seed_1` - 乱数の種
    /// * `_seed_2` - 乱数の種。`_seed_1`と同じ値の場合、コンストラクタ側で変更する。
    pub fn new(_seed_1: u32, _seed_2: u32) -> Self {
        let _seed_other = if _seed_1 != _seed_2 { _seed_2 } else { (_seed_1 as u64 + 1192u64) as u32};
        let xyzuv0: (u32, u32, u32, u32, u32) = create_state(_seed_1);
        let xyzuv1: (u32, u32, u32, u32, u32) = create_state(_seed_other);
        Self {
            x0: xyzuv0.0, y0: xyzuv0.1, z0: xyzuv0.2, u0: xyzuv0.3, v0: xyzuv0.4,
            x1: xyzuv1.0, y1: xyzuv1.1, z1: xyzuv1.2, u1: xyzuv1.3, v1: xyzuv1.4,
            scale: 1f64,
        }
    }

    /// 半コーシー分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        standard_cauchy(&mut self.x0, &mut self.y0, &mut self.z0, &mut self.u0, &mut self.v0,
            &mut self.x1, &mut self.y1, &mut self.z1, &mut self.u1, &mut self.v1).abs() * self.scale
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
/// 半コーシー分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed_1: expr, $seed_2: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut half_cauchy = rand_simple::create_half_cauchy!(1192u32, 765u32);
/// println!("尺度母数 θ = 1 の標準半コーシー分布に従う乱数を生成する -> {}", half_cauchy.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut half_cauchy = rand_simple::create_half_cauchy!();
/// println!("尺度母数 θ = 1 の標準半コーシー分布に従う乱数を生成する -> {}", half_cauchy.sample());
/// ```
macro_rules! create_half_cauchy {
    () => {{
        let seeds: (u32, u32) = $crate::create_seeds();
        $crate::HalfCauchy::new(seeds.0, seeds.1)
    }};
    ($seed_1: expr, $seed_2: expr) => {
        $crate::HalfCauchy::new($seed_1 as u32, $seed_2 as u32)
    };
}


impl std::fmt::Display for HalfCauchy {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}