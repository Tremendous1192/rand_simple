use crate::{Levy, set_state, update_and_uniform};
use std::cell::Cell;

impl Levy {
    /// コンストラクタ
    /// * `_seed_1` - 乱数の種
    /// * `_seed_2` - 乱数の種。`_seed_1`と同じ値の場合、コンストラクタ側で変更する。
    pub fn new(_seed_1: u32, _seed_2: u32) -> Self {
        let _seed_other = if _seed_1 != _seed_2 { _seed_2 } else { (_seed_1 as u64 + 1192u64) as u32};

        Self {
            xyzw_1: set_state(_seed_1),
            xyzw_2: set_state(_seed_other),
            even_flag: Cell::<bool>::new(false),
            even_result: Cell::<f64>::new(1f64),
        }
    }

    /// 標準レヴィ分布に従う乱数を返す
    /// * 位置母数 0
    /// * 尺度母数 1
    pub fn sample(&self) -> f64 {
        // アルゴリズム 3.40
        // step 1: 標準半正規分布HN(1)に従う乱数Zをz > 0の範囲で生成する
        // HN step 1 & 5: 偶数回目の乱数は、奇数回目で計算したもう一つの値を返す
        if self.even_flag.get() {
            self.even_flag.set(false);
            self.even_result.get()
        }
        else {
            loop {
                // HN step 2: 独立な一様乱数を2個生成する
                let u1: f64 = update_and_uniform(&self.xyzw_1);
                let u2: f64 = update_and_uniform(&self.xyzw_2);
                if u1 == 0f64 || u2 == 0f64 { continue; }

                // HN step 3: 中間変数を生成する
                let v = u1.powi(2) + u2.powi(2);

                // HN step 4: 0 < v < 1 のとき、乱数を計算する(v = 0は一様分布で弾いている)
                if v < 1f64 {
                    let w: f64 = (-2f64 * v.ln() / v).sqrt();

                    // step 2: 乱数を返す
                    self.even_result.set((u2 * w).powi(-2)); // x2
                    self.even_flag.set(true);
                    return (u1 * w).powi(-2); // x1
                }
            }
        }
    }
}

#[macro_export]
/// レヴィ分布のインスタンスを生成するマクロ
/// * `() =>` - 乱数の種は自動生成
/// * `($seed_1: expr, $seed_2: expr) =>` - 乱数の種を指定する
macro_rules! create_levy {
    () => {{
        let seeds: (u32, u32) = $crate::create_seeds();
        $crate::Levy::new(seeds.0, seeds.1)
    }};
    ($seed_1: expr, $seed_2: expr) => {
        $crate::Levy::new($seed_1 as u32, $seed_2 as u32)
    };
}