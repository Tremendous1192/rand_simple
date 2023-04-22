use std::cell::Cell;
use crate::{Bernoulli, set_parameters, calculate_uniform};

impl Bernoulli {
    /// コンストラクタ
    pub fn new(_seed: u32) -> Self {
        let (x, y, z, w) = set_parameters(_seed);

        Self {
            seed: _seed,
            x_cell: Cell::new(x),
            y_cell: Cell::new(y),
            z_cell: Cell::new(z),
            w_cell: Cell::new(w),
        }
    }

    /// 乱数の種を返す
    pub fn get_seed(&self) -> u32 {
        self.seed
    }

    /// 乱数を返す
    /// theta: ある事象が生じる確率
    pub fn next_uint(&self, theta: f64) -> u32 {
        let (x, y, z, w, f) 
        = calculate_uniform(
            self.x_cell.get(), self.y_cell.get(),
            self.z_cell.get(), self.w_cell.get());

        // 状態を記録する
        self.x_cell.set(x);
        self.y_cell.set(y);
        self.z_cell.set(z);
        self.w_cell.set(w);

        // 確率変数を計算する()
        if f <= theta { 1u32 }
        else { 0u32 }
    }
}