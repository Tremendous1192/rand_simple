use crate::standard_distributions::{generate_random_state, xorshift160_0_to_1};

/// Geometric Distribution
///
/// # Example Usage
/// ```
/// let mut geometric = rand_simple::Geometric::new(1192u32);
///
/// // Default parameters
/// assert_eq!(format!("{geometric}"), "Geometric(Probability) = Geometric(0.5)");
/// println!("Number of trials until success with probability θ = 0.5 -> {}", geometric.sample());
///
/// // Updating the probability parameter
/// let probability: f64 = 0.8f64;
/// let result: Result<f64, &str> = geometric.try_set_params(probability);
/// assert_eq!(format!("{geometric}"), "Geometric(Probability) = Geometric(0.8)");
/// println!("Number of trials until success with probability θ = {} -> {}", probability, geometric.sample());
/// ```
pub struct Geometric {
    xyzuv: [u32; 5],  // 状態変数
    probability: f64, // 発生確率
}

impl Geometric {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzuv: [u32; 5] = generate_random_state(_seed);
        Self {
            xyzuv,
            probability: 0.5_f64,
        }
    }

    /// ある確率の事象が初めて生じるまでの試行回数を返す
    pub fn sample(&mut self) -> u64 {
        let mut x: u64 = 1_u64;
        while xorshift160_0_to_1(&mut self.xyzuv) > self.probability {
            x += 1_u64;
        }
        x
    }

    /// 確率変数のパラメータを変更する
    /// * `probability` - 発生確率
    pub fn try_set_params(&mut self, probability: f64) -> Result<f64, &str> {
        if !(0_f64..=1_f64).contains(&probability) {
            Err("発生確率が0より小さいか、1よりも大きいです。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.probability = probability;
            Ok(probability)
        }
    }
}

impl core::fmt::Display for Geometric {
    /// Formatter for displaying in functions like println! macro
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Geometric(Probability) = Geometric({})",
            self.probability
        )?;
        Ok(())
    }
}
