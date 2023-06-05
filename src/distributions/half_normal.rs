use crate::{HalfNormal, initialize, update};
use std::cell::Cell;

impl HalfNormal {
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
            even_flag: Cell::<bool>::new(false),
            even_result: Cell::<f64>::new(0f64),
            std: Cell::new(1f64),
        }
    }

    /// 標準半正規分布に従う乱数を返す
    pub fn sample(&self) -> f64 {
        // アルゴリズム 3.17
        // step 1 & 5: 偶数回目の乱数は、奇数回目で計算したもう一つの値を返す
        if self.even_flag.get() {
            self.even_flag.set(false);
            self.even_result.get()
        }
        else {
            loop {
                // step 2: 独立な一様乱数を2個生成する
                let u1: f64 = update(&self.x0, &self.y0, &self.z0, &self.w0);
                let u2: f64 = update(&self.x1, &self.y1, &self.z1, &self.w1);

                // step 3: 中間変数を生成する
                let v = u1.powi(2) + u2.powi(2);

                // step 4: 0 < v < 1 のとき、乱数を計算する
                if 0f64 < v && v < 1f64 {
                    let w: f64 = (-2f64 * v.ln() / v).sqrt();

                    // step 5: 計算した乱数を返す
                    self.even_result.set(u2 * w * self.std.get()); // y2
                    self.even_flag.set(true);
                    return u1 * w * self.std.get(); // y1
                }
            }
        }
    }

    /// 確率変数のパラメータを変更する
    /// * `variance` - 分散
    pub fn try_set_params(&self, variance: f64) -> Result<f64, &str> {
        if variance < 0f64 {
            Err("分散が0以下です。確率変数のパラメータは前回の設定を維持します。")
        }
        else {
            self.std.set(variance.sqrt());
            Ok( self.std.get().powi(2) )
        }
    }
}

#[macro_export]
/// 半正規分布のインスタンスを生成するマクロ
/// * `() =>` - 乱数の種は自動生成
/// * `($seed_1: expr, $seed_2: expr) =>` - 乱数の種を指定する
/// # 使用例 1
/// ```
/// let half_normal = rand_simple::create_half_normal!(1192u32, 765u32);
/// assert_eq!(half_normal.sample(), 1.8943489630074781f64);
/// ```
/// # 使用例 2
/// ```
/// let half_normal = rand_simple::create_half_normal!();
/// println!("乱数: {}", half_normal.sample()); // インスタンス生成時刻に依存するため、コンパイル時は値不明
/// ```
macro_rules! create_half_normal {
    () => {{
        let seeds: (u32, u32) = $crate::create_seeds();
        $crate::HalfNormal::new(seeds.0, seeds.1)
    }};
    ($seed_1: expr, $seed_2: expr) => {
        $crate::HalfNormal::new($seed_1 as u32, $seed_2 as u32)
    };
}


impl std::fmt::Display for HalfNormal {
    /// println!マクロなどで表示するためのフォーマッタ
    /// * 構造体の型
    /// * 分散
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "構造体の型: {}", std::any::type_name::<Self>())?;
        writeln!(f, "分散: {}", self.std.get().powi(2))?;
        Ok(())
    }
}