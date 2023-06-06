use crate::{HalfCauchy, initialize, update};
use std::cell::Cell;

impl HalfCauchy {
    /// コンストラクタ
    /// * `_seed_1` - 乱数の種
    /// * `_seed_2` - 乱数の種。`_seed_1`と同じ値の場合、コンストラクタ側で変更する。
    pub fn new(_seed_1: u32, _seed_2: u32) -> Self {
        let _seed_other = if _seed_1 != _seed_2 { _seed_2 } else { (_seed_1 as u64 + 1192u64) as u32};
        let xyzw0: (u32, u32, u32, u32) = initialize(_seed_1);
        let xyzw1: (u32, u32, u32, u32) = initialize(_seed_other);
        Self {
            x0: Cell::new(xyzw0.0), y0: Cell::new(xyzw0.1), z0: Cell::new(xyzw0.2), w0: Cell::new(xyzw0.3),
            x1: Cell::new(xyzw1.0), y1: Cell::new(xyzw1.1), z1: Cell::new(xyzw1.2), w1: Cell::new(xyzw1.3),
            scale: Cell::new(1f64),
        }
    }

    /// 半コーシー分布に従う乱数を返す
    pub fn sample(&self) -> f64 {
        // アルゴリズム 3.34
        loop {
            // step 1: 区間(0, 1) の一様乱数u1, 区間(0, 1)のu2を独立に発生させる。
            let u1: f64 = update(&self.x0, &self.y0, &self.z0, &self.w0);
            let u2: f64 = update(&self.x1, &self.y1, &self.z1, &self.w1);
            if u1 == 1f64 || u2 == 0f64 || u2 == 1f64 { continue; }

            // step 2: 中間変数を生成する
            let w = u1.powi(2) + u2.powi(2);

            // step 3: w < 1のとき、戻り値計算に移る
            if w < 1f64 {
                return u1 / u2 * self.scale.get();
            }
        }
    }

    /// 確率変数のパラメータを変更する
    /// * `scale` - 尺度母数
    pub fn try_set_params(&self, scale: f64) -> Result<f64, &str> {
        if scale <= 0f64 {
            Err("尺度母数が0以下です。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.scale.set(scale);
            Ok( self.scale.get() )
        }
    }
}

#[macro_export]
/// 半コーシー分布
/// * `() =>` - 乱数の種は自動生成
/// * `($seed_1: expr, $seed_2: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let half_cauchy = rand_simple::create_half_cauchy!(1192u32, 765u32);
/// assert_eq!(half_cauchy.sample(), 0.9999971261133705f64);
/// ```
/// # 使用例 2
/// ```
/// let half_cauchy = rand_simple::create_half_cauchy!();
/// println!("乱数: {}", half_cauchy.sample()); // インスタンス生成時刻に依存するため、コンパイル時は値不明
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
        writeln!(f, "尺度母数: {}", self.scale.get())?;
        Ok(())
    }
}