use crate::{Geometric, set_state, update_and_uniform};

impl Geometric {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzw: set_state(_seed),
        }
    }

    /// ある確率の事象が初めて生じるまでの試行回数を返す
    /// * `probability` - ある事象が生じる確率
    pub fn sample(&self, probability: f64) -> u32 {
        let mut trial: u32 = 1;

        loop {
            if update_and_uniform(&self.xyzw) <= probability { return trial; }
            else { trial += 1; }
        }
    }
}

#[macro_export]
/// 幾何分布のインスタンスを生成するマクロ
macro_rules! create_geometric {
    // 引数無し
    () => {{
        $crate::Geometric::new($crate::create_seed())
    }};
    // 引数有り
    ($seed: expr) => {
        $crate::Geometric::new($seed as u32)
    };
}