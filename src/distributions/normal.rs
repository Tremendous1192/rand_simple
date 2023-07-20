use crate::standard_distributions::standard_normal;
use crate::{create_state, Normal};

impl Normal {
    /// コンストラクタ
    /// * `_seed_*` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(_seed_1: u32, _seed_2: u32) -> Self {
        let seeds = crate::adjust_values!(_seed_1, _seed_2);
        Self {
            xyzuv0: create_state(seeds[0]),
            xyzuv1: create_state(seeds[1]),
            mean: 0f64,
            std: 1f64,
        }
    }

    /// 正規分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        standard_normal(&mut self.xyzuv0, &mut self.xyzuv1) * self.std + self.mean
    }

    /// 確率変数のパラメータを変更する
    /// * `mean` - 平均
    /// * `variance` - 分散
    pub fn try_set_params(&mut self, mean: f64, variance: f64) -> Result<(f64, f64), &str> {
        if variance <= 0f64 {
            Err("分散が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.mean = mean;
            self.std = variance.sqrt();
            Ok((mean, variance))
        }
    }
}

#[macro_export]
/// 正規分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed_1: expr, $seed_2: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut normal = rand_simple::create_normal!(1192u32, 765u32);
/// println!("平均値 0, 分散 1 の標準正規分布乱数を生成する -> {}", normal.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut normal = rand_simple::create_normal!();
/// println!("平均値 0, 分散 1 の標準正規分布乱数を生成する -> {}", normal.sample());
/// ```
macro_rules! create_normal {
    () => {{
        let seeds: (u32, u32) = $crate::create_seeds();
        $crate::Normal::new(seeds.0, seeds.1)
    }};
    ($seed_1: expr, $seed_2: expr) => {
        $crate::Normal::new($seed_1 as u32, $seed_2 as u32)
    };
}

impl std::fmt::Display for Normal {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 平均
    /// * 分散
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "平均: {}", self.mean)?;
        writeln!(f, "分散: {}", self.std.powi(2))?;
        Ok(())
    }
}
