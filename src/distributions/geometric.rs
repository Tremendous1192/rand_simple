use crate::{Geometric, initialize, update};
use std::cell::Cell;

impl Geometric {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzw: (u32, u32, u32, u32) = initialize(_seed);
        Self {
            x: Cell::new(xyzw.0), y: Cell::new(xyzw.1), z: Cell::new(xyzw.2), w: Cell::new(xyzw.3),
        }
    }

    /// ある確率の事象が初めて生じるまでの試行回数を返す
    /// * `probability` - ある事象が生じる確率
    pub fn sample(&self, probability: f64) -> u64 {
        // オーバーフロー対策
        if probability < 0f64 {return std::u64::MAX};

        // アルゴリズム 4.13
        // step 1: x = 1と初期設定する
        let mut x: u64 = 1;

        // step 2: 区間[0, 1]の一様乱数uを発生させる
        // step 3: u ≦ θ(発生確率)のときxを所望の乱数として出力する
        // u > θのときはxの値を1増やしてstep 2に戻る
        while update(&self.x, &self.y, &self.z, &self.w) > probability { x += 1u64; }        
        x
    }
}

#[macro_export]
/// 幾何分布のインスタンスを生成するマクロ
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
macro_rules! create_geometric {
    () => {
        $crate::Geometric::new($crate::create_seed())
    };
    ($seed: expr) => {
        $crate::Geometric::new($seed as u32)
    };
}