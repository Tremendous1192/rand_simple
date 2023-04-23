use std::cell::Cell;
use crate::{Uniform, set_parameters, update_state_and_calculate_uniform};

impl Uniform {
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

    /// 閉区間[0, 1]の乱数を返す
    pub fn sample(&self) -> f64 {
        update_state_and_calculate_uniform(&self.x_cell, &self.y_cell, &self.z_cell, &self.w_cell)
    }

}


