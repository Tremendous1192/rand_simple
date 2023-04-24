use crate::{Bernoulli, set_state, update_and_uniform};

impl Bernoulli {
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

    /// 乱数を返す
    /// 
    /// probability: ある事象が生じる確率
    pub fn sample(&self, probability: f64) -> u32 {
        let f: f64 = update_and_uniform(&self.xyzw);

        // 確率変数を計算する()
        if f <= probability { 1u32 }
        else { 0u32 }
    }
}