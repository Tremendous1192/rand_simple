use crate::standard_distributions::standard_normal;
use crate::{create_state, HalfNormal};

impl HalfNormal {
    /// コンストラクタ
    /// * `_seed_*` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(_seed_1: u32, _seed_2: u32) -> Self {
        let seeds = crate::adjust_values!(_seed_1,_seed_2);
        Self {
            xyzuv0:create_state(seeds[0]),
            xyzuv1:create_state(seeds[1]),
            std: 1f64,
        }
    }

    /// 標準半正規分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        standard_normal(&mut self.xyzuv0, &mut self.xyzuv1).abs() * self.std
    }

    /// 確率変数のパラメータを変更する
    /// * `variance` - 分散
    pub fn try_set_params(&mut self, variance: f64) -> Result<f64, &str> {
        if variance <= 0f64 {
            Err("分散が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.std = variance.sqrt();
            Ok(variance)
        }
    }
}

#[macro_export]
/// 半正規分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed_1: expr, $seed_2: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut half_normal = rand_simple::create_half_normal!(1192u32, 765u32);
/// println!("分散 1 の標準半正規分布乱数を生成する -> {}", half_normal.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut half_normal = rand_simple::create_half_normal!();
/// println!("分散 1 の半正規分布乱数を生成する -> {}", half_normal.sample());
/// ```
macro_rules! create_half_normal {
    () => {{
        let seeds: (u32, u32) = $crate::create_seeds();
        $crate::HalfNormal::new(seeds.0, seeds.1)
    }};
    ($seed_1: expr, $seed_2: expr) => {
        $crate::HalfNormal::new($seed_1 as u32, $seed_2 as u32)
    };
}

impl std::fmt::Display for HalfNormal {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 分散
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "分散: {}", self.std.powi(2))?;
        Ok(())
    }
}
