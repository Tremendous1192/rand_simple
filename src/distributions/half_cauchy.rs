use crate::standard_distributions::standard_cauchy;
use crate::{create_state, HalfCauchy};

impl HalfCauchy {
    /// コンストラクタ
    /// * `_seed_*` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(_seed_1: u32, _seed_2: u32) -> Self {
        let seeds = crate::adjust_values!(_seed_1, _seed_2);
        Self {
            xyzuv0: create_state(seeds[0]),
            xyzuv1: create_state(seeds[1]),
            scale: 1f64,
        }
    }

    /// 半コーシー分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        standard_cauchy(&mut self.xyzuv0, &mut self.xyzuv1).abs() * self.scale
    }

    /// 確率変数のパラメータを変更する
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, scale: f64) -> Result<f64, &str> {
        if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.scale = scale;
            Ok(scale)
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
