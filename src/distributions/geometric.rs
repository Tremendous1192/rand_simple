use std::cell::Cell;
use crate::{Geometric, set_parameters, update_state_and_calculate_uniform};

impl Geometric {
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
    /// 
    /// probability: ある事象が生じる確率
    pub fn sample(&self, probability: f64) -> u32 {
        let mut trial: u32 = 1;

        loop {
            let f: f64 = update_state_and_calculate_uniform(&self.x_cell, &self.y_cell, &self.z_cell, &self.w_cell);

            // 確率変数を計算する()
            if f <= probability { break; }
            else { trial += 1; }
        }

        trial
    }
}