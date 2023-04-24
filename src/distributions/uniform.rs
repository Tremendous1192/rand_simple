use crate::{Uniform, set_state, update_and_uniform};

impl Uniform {
    /// コンストラクタ
    pub fn new(_seed: u32) -> Self {
        Self {
            seed: _seed,
            xyzw: set_state(_seed),
        }
    }

    /// 乱数の種を返す
    pub fn get_seed(&self) -> u32 {
        self.seed
    }

    /// 閉区間[0, 1]の乱数を返す
    pub fn sample(&self) -> f64 {
        update_and_uniform(&self.xyzw)
    }

}


