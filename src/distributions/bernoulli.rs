use crate::standard_distributions::{generate_random_state, xorshift160_0_to_1};

/// Bernoulli Distribution
///
/// # Example Usage
/// ```
/// let mut bernoulli = rand_simple::Bernoulli::new(1192u32);
///
/// // Default parameters
/// assert_eq!(format!("{bernoulli}"), "Bernoulli(Probability) = Bernoulli(0.5)");
/// println!("Does the event occur (1) or not (0) with a probability θ = 0.5? -> {}", bernoulli.sample());
///
/// // Updating the probability parameter
/// let probability: f64 = 0.8f64;
/// let result: Result<f64, &str> = bernoulli.try_set_params(probability);
/// assert_eq!(format!("{bernoulli}"), "Bernoulli(Probability) = Bernoulli(0.8)");
/// println!("Does the event occur (1) or not (0) with a probability θ = {}? -> {}", probability, bernoulli.sample());
/// ```
pub struct Bernoulli {
    xyzuv: [u32; 5],  // 状態変数
    probability: f64, // 発生確率
}

impl Bernoulli {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        let xyzuv: [u32; 5] = generate_random_state(_seed);
        Self {
            xyzuv,
            probability: 0.5_f64,
        }
    }

    /// ある確率の事象が生じたか(1u64)、否か(0u64)を返す
    pub fn sample(&mut self) -> u64 {
        if xorshift160_0_to_1(&mut self.xyzuv) <= self.probability {
            1_u64
        } else {
            0_u64
        }
    }

    /// 確率変数のパラメータを変更する
    /// * `probability` - 尺度母数
    pub fn try_set_params(&mut self, probability: f64) -> Result<f64, &str> {
        if !(0_f64..=1_f64).contains(&probability) {
            Err("発生確率が0より小さいか、1よりも大きいです。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.probability = probability;
            Ok(probability)
        }
    }
}

impl core::fmt::Display for Bernoulli {
    /// Formatter for displaying in functions like println! macro
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Bernoulli(Probability) = Bernoulli({})",
            self.probability
        )?;
        Ok(())
    }
}
