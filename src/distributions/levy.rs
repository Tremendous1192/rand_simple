use crate::standard_distributions::standard_normal;
use crate::{create_state, Levy};

impl Levy {
    /// コンストラクタ
    /// * `_seed_1` - 乱数の種
    /// * `_seed_2` - 乱数の種。`_seed_1`と同じ値の場合、コンストラクタ側で変更する。
    pub fn new(_seed_1: u32, _seed_2: u32) -> Self {
        let _seed_other = if _seed_1 != _seed_2 {
            _seed_2
        } else {
            (_seed_1 as u64 + 1192u64) as u32
        };
        let xyzuv0: [u32; 5] = create_state(_seed_1);
        let xyzuv1: [u32; 5] = create_state(_seed_other);
        Self {
            xyzuv0,
            xyzuv1,
            location: 0f64,
            scale: 1f64,
        }
    }

    /// レヴィ分布に従う乱数を返す
    pub fn sample(&mut self) -> f64 {
        loop {
            let z = standard_normal(&mut self.xyzuv0, &mut self.xyzuv1).abs();
            if z > 0f64 {
                return z.powi(-2) * self.scale + self.location;
            }
        }
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

#[macro_export]
/// レヴィ分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed_1: expr, $seed_2: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let mut levy = rand_simple::create_levy!(1192u32, 765u32);
/// println!("位置母数 μ = 0, 尺度母数 θ = 1 の標準レヴィ分布に従う乱数を生成する -> {}", levy.sample());
/// ```
/// # 使用例 2
/// ```
/// let mut levy = rand_simple::create_levy!();
/// println!("位置母数 μ = 0, 尺度母数 θ = 1 の標準レヴィ分布に従う乱数を生成する -> {}", levy.sample());
/// ```
macro_rules! create_levy {
    () => {{
        let seeds: (u32, u32) = $crate::create_seeds();
        $crate::Levy::new(seeds.0, seeds.1)
    }};
    ($seed_1: expr, $seed_2: expr) => {
        $crate::Levy::new($seed_1 as u32, $seed_2 as u32)
    };
}

impl std::fmt::Display for Levy {
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
