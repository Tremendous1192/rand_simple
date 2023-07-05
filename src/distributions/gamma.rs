use crate::standard_distributions::{standard_gamma, xorshift160_0_1_open};
use crate::{create_state, Gamma};

impl Gamma {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed_1: u32, _seed_2: u32, _seed_3: u32) -> Self {
        let mut xyzuv: [u32; 5] = create_state(_seed_1);
        let u_1: f64 = xorshift160_0_1_open(&mut xyzuv);
        let _seed_other = if _seed_2 != _seed_3 {
            _seed_3
        } else {
            (_seed_2 as u64 + 1192u64) as u32
        };
        let xyzuv0: [u32; 5] = create_state(_seed_2);
        let xyzuv1: [u32; 5] = create_state(_seed_other);
        Self {
            xyzuv,
            previous_uniform_1: u_1,
            xyzuv0,
            xyzuv1,
            shape: 1f64,
            scale: 1f64,
        }
    }

    /// ガンマ分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        standard_gamma(
            &mut self.xyzuv,
            &mut self.previous_uniform_1,
            &mut self.xyzuv0,
            &mut self.xyzuv1,
            &self.shape,
        ) * self.scale
    }

    /// 確率変数のパラメータを変更する
    /// * `shape` - 形状母数
    /// * `scale` - 尺度母数
    pub fn try_set_params(&mut self, shape: f64, scale: f64) -> Result<(f64, f64), &str> {
        if shape <= 0f64 {
            Err("形状母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else if shape == 1f64 / 3f64 {
            Err("形状母数が1/3です。確率変数のパラメータは前回の設定を維持します。")
        } else if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.shape = shape;
            self.scale = scale;
            Ok((shape, scale))
        }
    }
}

#[macro_export]
/// ガンマ分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed_0: expr, $seed_1: expr, $seed_2: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut gamma = rand_simple::create_gamma!(1192u32, 765u32, 1543u32);
/// println!("形状母数 α = 1, 尺度母数 β = 1の標準ガンマ分布に従う乱数を生成する -> {}", gamma.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut gamma = rand_simple::create_gamma!();
/// println!("形状母数 α = 1, 尺度母数 β = 1の標準ガンマ分布に従う乱数を生成する -> {}", gamma.sample());
/// ```
macro_rules! create_gamma {
    () => {{
        let seeds: (u32, u32, u32) = $crate::create_seeds_trio();
        $crate::Gamma::new(seeds.0, seeds.1, seeds.2)
    }};
    ($seed_0: expr, $seed_1: expr, $seed_2: expr) => {
        $crate::Gamma::new($seed_0 as u32, $seed_1 as u32, $seed_2 as u32)
    };
}

impl std::fmt::Display for Gamma {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 尺度母数
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "形状母数: {}", self.shape)?;
        writeln!(f, "尺度母数: {}", self.scale)?;
        Ok(())
    }
}
