use crate::{Cauchy, set_state, update_and_uniform};

impl Cauchy {
    /// コンストラクタ
    /// * `_seed_1` - 乱数の種
    /// * `_seed_2` - 乱数の種。`_seed_1`と同じ値の場合、コンストラクタ側で変更する。
    pub fn new(_seed_1: u32, _seed_2: u32) -> Self {
        let _seed_other = if _seed_1 != _seed_2 { _seed_2 } else { (_seed_1 as u64 + 1192u64) as u32};

        Self {
            xyzw_1: set_state(_seed_1),
            xyzw_2: set_state(_seed_other),
        }
    }

    /// 標準コーシー分布に従う乱数を返す
    /// * 位置母数 0
    /// * 尺度母数 1
    pub fn sample(&self) -> f64 {
        // アルゴリズム 3.27
        loop {
            // step 1: 区間(0, 1) の一様乱数u1, u2を独立に発生させる。ただし、u2 ≒ 0.5
            let u1: f64 = update_and_uniform(&self.xyzw_1);
            let u2: f64 = update_and_uniform(&self.xyzw_2);
            if u2 == 0.5f64 { continue; }

            // step 2: 中間変数を生成する
            let v1 = 2f64 * u1 - 1f64;
            let v2 = 2f64 * u2 - 1f64;
            let w = v1.powi(2) + v2.powi(2);

            // step 3: w < 1のとき、戻り値計算に移る
            if w < 1f64 {
                return v1 / v2;
            }
        }
    }
}

#[macro_export]
/// コーシー分布のインスタンスを生成するマクロ
/// * `() =>` - 乱数の種は自動生成
/// * `($seed_1: expr, $seed_2: expr) =>` - 乱数の種を指定する
macro_rules! create_cauchy {
    () => {{
        let seeds: (u32, u32) = $crate::create_seeds();
        $crate::Cauchy::new(seeds.0, seeds.1)
    }};
    ($seed_1: expr, $seed_2: expr) => {
        $crate::Cauchy::new($seed_1 as u32, $seed_2 as u32)
    };
}