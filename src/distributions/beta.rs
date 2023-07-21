use crate::standard_distributions::{standard_gamma, xorshift160_0_1_open};
use crate::{create_state, Beta};

impl Beta {
    /// コンストラクタ
    /// * `_seed_i` - 乱数の種。同じ値にならないようにコンストラクタ側で調整する。
    pub fn new(
        _seed_0: u32,
        _seed_1: u32,
        _seed_2: u32,
        _seed_3: u32,
        _seed_4: u32,
        _seed_5: u32,
    ) -> Self {
        let seeds = crate::adjust_values!(_seed_0, _seed_1, _seed_2, _seed_3, _seed_4, _seed_5);

        let mut xyzuv_alpha: [u32; 5] = create_state(seeds[0]);
        let previous_uniform_1_alpha: f64 = xorshift160_0_1_open(&mut xyzuv_alpha);

        let mut xyzuv_beta: [u32; 5] = create_state(seeds[3]);
        let previous_uniform_1_beta: f64 = xorshift160_0_1_open(&mut xyzuv_beta);

        Self {
            xyzuv_alpha,
            previous_uniform_1_alpha,
            xyzuv0_alpha: create_state(seeds[1]),
            xyzuv1_alpha: create_state(seeds[2]),
            shape_alpha: 1f64,

            xyzuv_beta,
            previous_uniform_1_beta,
            xyzuv0_beta: create_state(seeds[4]),
            xyzuv1_beta: create_state(seeds[5]),
            shape_beta: 1f64,
        }
    }

    /// ベータ分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        let y1 = standard_gamma(
            &mut self.xyzuv_alpha,
            &mut self.previous_uniform_1_alpha,
            &mut self.xyzuv0_alpha,
            &mut self.xyzuv1_alpha,
            &self.shape_alpha,
        );
        let y2 = standard_gamma(
            &mut self.xyzuv_beta,
            &mut self.previous_uniform_1_beta,
            &mut self.xyzuv0_beta,
            &mut self.xyzuv1_beta,
            &self.shape_beta,
        );
        y1 / (y1 + y2)
    }

    /// 確率変数のパラメータを変更する
    /// * `shape_alpha` - 形状母数 α
    /// * `shape_beta` - 形状母数 β
    pub fn try_set_params(
        &mut self,
        shape_alpha: f64,
        shape_beta: f64,
    ) -> Result<(f64, f64), &str> {
        if shape_alpha <= 0f64 {
            Err("形状母数 α が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape_alpha == 1f64 / 3f64 {
            Err("形状母数 α が1/3です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape_beta <= 0f64 {
            Err("形状母数 β が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape_beta == 1f64 / 3f64 {
            Err("形状母数 β が1/3です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape_alpha = shape_alpha;
            self.shape_beta = shape_beta;
            Ok((shape_alpha, shape_beta))
        }
    }
}

#[macro_export]
/// ベータ分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed_0: expr, $seed_1: expr, $seed_2: expr, $seed_3: expr, $seed_4: expr, $seed_5: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut beta = rand_simple::create_beta!(1192u32, 765u32, 1543u32, 2003u32, 1867u32, 1688u32);
/// println!("形状母数 α = 1, 形状母数 β = 1の標準ベータ分布に従う乱数を生成する -> {}", beta.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut beta = rand_simple::create_beta!();
/// println!("形状母数 α = 1, 形状母数 β = 1の標準ベータ分布に従う乱数を生成する -> {}", beta.sample());
/// ```
macro_rules! create_beta {
    () => {{
        let seeds: (u32, u32, u32) = $crate::create_seeds_trio();
        let seeds1 = (seeds.1 + 1192u32, seeds.2 + 765u32, seeds.0 + 1991u32);
        $crate::Beta::new(seeds.0, seeds.1, seeds.2, seeds1.0, seeds1.1, seeds1.2)
    }};
    ($seed_0: expr, $seed_1: expr, $seed_2: expr, $seed_3: expr, $seed_4: expr, $seed_5: expr) => {
        $crate::Beta::new(
            $seed_0 as u32,
            $seed_1 as u32,
            $seed_2 as u32,
            $seed_3 as u32,
            $seed_4 as u32,
            $seed_5 as u32,
        )
    };
}

impl std::fmt::Display for Beta {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "形状母数 α: {}", self.shape_alpha)?;
        writeln!(f, "形状母数 β: {}", self.shape_beta)?;
        Ok(())
    }
}
