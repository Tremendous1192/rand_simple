use crate::{Normal, set_state, update_and_uniform};
use std::cell::Cell;

impl Normal {
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

    /// 平均値 0, 標準偏差 1 の標準正規分布乱数を返す
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
                let v1 = 2f64 * u1 - 1f64;
                let v2 = 2f64 * u2 - 1f64;
                let v = v1 * v1 + v2 * v2;

                // step 4: 0 < v < 1 のとき、乱数を計算する
                if 0f64 < v && v < 1f64 {
                    let w: f64 = (-2f64 * v.ln() / v).sqrt();
                    self.even_result.set(v2 * w); // y2

                    // step 5: 計算した乱数を返す
                    self.even_flag.set(true);
                    return v1 * w; // y1
                }
            }
        }
    }
}

#[macro_export]
/// 正規分布のインスタンスを生成するマクロ
macro_rules! create_normal {
    // 引数無し
    () => {{
        let seeds: (u32, u32) = $crate::create_seeds();
        $crate::Normal::new(seeds.0, seeds.1)
    }};
    // 引数有り
    ($seed_1: expr, $seed_2: expr) => {
        $crate::Normal::new($seed_1 as u32, $seed_2 as u32)
    };
}