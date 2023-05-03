use crate::{HalfCauchy, set_state, update_and_uniform};

impl HalfCauchy {
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

    /// 平均値 0, 標準偏差 1 の標準正規分布乱数を返す
    pub fn sample(&self) -> f64 {
        loop {
            // step 1: 区間(0, 1) の一様乱数u1, u2を独立に発生させる。ただし、u2 ≒ 0.5
            let u1: f64 = update_and_uniform(&self.xyzw_1);
            let u2: f64 = update_and_uniform(&self.xyzw_2);

            if 0f64 < u1 && u1 < 1f64 && 0f64 < u2 && u2 < 1f64 {
                // step 2: 中間変数を生成する
                let w = u1 * u1 + u2 * u2;

                // step 3: w < 1のとき、乱数を計算する
                if w < 1f64 {
                    return u1 / u2;
                }

            }
        }
    }
}

#[macro_export]
/// 半コーシー分布のインスタンスを生成するマクロ
macro_rules! create_half_cauchy {
    // 引数無し
    () => {{
        let seeds: (u32, u32) = $crate::create_seeds();
        $crate::HalfCauchy::new(seeds.0, seeds.1)
    }};
    // 引数有り
    ($seed_1: expr, $seed_2: expr) => {
        $crate::HalfCauchy::new($seed_1 as u32, $seed_2 as u32)
    };
}