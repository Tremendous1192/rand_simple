use crate::standard_distributions::standard_cauchy;
use crate::{create_state, Cauchy};

impl Cauchy {
    /// コンストラクタ
    /// * `seeds` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(seeds: [u32; 2]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv0: create_state(adjusted_seeds[0]),
            xyzuv1: create_state(adjusted_seeds[1]),
            location: 0f64,
            scale: 1f64,
        }
    }

    /// コーシー分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        standard_cauchy(&mut self.xyzuv0, &mut self.xyzuv1) * self.scale + self.location
    }

    /// 確率変数のパラメータを変更する
    /// * `location` - 位置母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, location: f64, scale: f64) -> Result<(f64, f64), &str> {
        if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.location = location;
            self.scale = scale;
            Ok((location, scale))
        }
    }
}

/*
#[macro_export]
/// コーシー分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed_1: expr, $seed_2: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut cauchy = rand_simple::create_cauchy!(1192u32, 765u32);
/// println!("位置母数 μ = 0, 尺度母数 θ = 1の標準コーシー分布に従う乱数を生成する -> {}", cauchy.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut cauchy = rand_simple::create_cauchy!();
/// println!("位置母数 μ = 0, 尺度母数 θ = 1の標準コーシー分布に従う乱数を生成する -> {}", cauchy.sample());
/// ```
macro_rules! create_cauchy {
    () => {{
        let seeds: (u32, u32) = $crate::create_seeds();
        $crate::Cauchy::new(seeds.0, seeds.1)
    }};
    ($seed_1: expr, $seed_2: expr) => {
        $crate::Cauchy::new($seed_1 as u32, $seed_2 as u32)
    };
}
*/

impl std::fmt::Display for Cauchy {
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
