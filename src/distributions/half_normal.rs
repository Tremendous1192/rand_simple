use crate::{HalfNormal, set_state, update_and_uniform};
use std::cell::Cell;

impl HalfNormal {
    /// コンストラクタ
    /// * `_seed_1` - 乱数の種
    /// * `_seed_2` - 乱数の種。`_seed_1`と同じ値の場合、コンストラクタ側で変更する。
    pub fn new(_seed_1: u32, _seed_2: u32) -> Self {
        let _seed_other = if _seed_1 != _seed_2 { _seed_2 } else { (_seed_1 as u64 + 1192u64) as u32};

        Self {
            xyzw_1: set_state(_seed_1),
            xyzw_2: set_state(_seed_other),
            even_flag: Cell::<bool>::new(false),
            even_result: Cell::<f64>::new(0f64),
        }
    }

    /// 標準偏差 1 の標準正規分布乱数を返す
    pub fn sample(&self) -> f64 {
        if self.even_flag.get() {
            // step 1 & 5: 偶数回目の乱数は、奇数回目で計算したもう一つの値を返す
            self.even_flag.set(false);
            self.even_result.get()
        }
        else {
            loop {
                // step 2: 独立な一様乱数を2個生成する
                let u1: f64 = update_and_uniform(&self.xyzw_1);
                let u2: f64 = update_and_uniform(&self.xyzw_2);

                // step 3: 中間変数を生成する
                let v = u1 * u1 + u2 * u2;

                // step 4: 0 < v < 1 のとき、乱数を計算する
                if 0f64 < v && v < 1f64 {
                    let w: f64 = (-2f64 * v.ln() / v).sqrt();
                    self.even_result.set(u2 * w); // y2

                    // step 5: 計算した乱数を返す
                    self.even_flag.set(true);
                    return u1 * w; // y1
                }
            }
        }
    }
}

#[macro_export]
/// 半正規分布のインスタンスを生成するマクロ
macro_rules! create_half_normal {
    // 引数無し
    () => {{
        let seeds: (u32, u32) = $crate::create_seeds();
        $crate::HalfNormal::new(seeds.0, seeds.1)
    }};
    // 引数有り
    ($seed_1: expr, $seed_2: expr) => {
        $crate::HalfNormal::new($seed_1 as u32, $seed_2 as u32)
    };
}