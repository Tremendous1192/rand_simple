use crate::standard_distributions::{generate_random_state, standard_normal};

/// Normal Distribution
/// # Example
/// ```
/// let mut normal = rand_simple::Normal::new([1192_u32, 765_u32]);
/// assert_eq!(format!("{normal}"), "N(Mean, Std^2) = N(0, 1^2)");
/// println!("Returns a random number -> {}", normal.sample());
///
/// // If you want to change the parameters of the random variable
/// let mean: f64 = -3_f64;
/// let std: f64 = 2_f64;
/// let result: Result<(f64, f64), &str> = normal.try_set_params(mean, std);
/// assert_eq!(format!("{normal}"), "N(Mean, Std^2) = N(-3, 2^2)");
/// println!("Returns a random number -> {}", normal.sample());
/// ```
pub struct Normal {
    xyzuv0: [u32; 5], // 状態変数
    xyzuv1: [u32; 5], // 状態変数
    mean: f64,        // 平均
    std: f64,         // 標準偏差
}

impl Normal {
    /// Constructor
    /// * `seeds` - Seeds for random number generation. Adjusted on the constructor side to ensure they are not the same.
    pub fn new(seeds: [u32; 2]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv0: generate_random_state(adjusted_seeds[0]),
            xyzuv1: generate_random_state(adjusted_seeds[1]),
            mean: 0_f64,
            std: 1_f64,
        }
    }

    /// Generate a random number.
    pub fn sample(&mut self) -> f64 {
        standard_normal(&mut self.xyzuv0, &mut self.xyzuv1) * self.std + self.mean
    }

    /// Modify the parameters of the random variable.
    /// * `mean` - Mean
    /// * `std` - Standard deviation
    pub fn try_set_params(&mut self, mean: f64, std: f64) -> Result<(f64, f64), &'static str> {
        if std <= 0_f64 {
            Err("Standard deviation is less than or equal to 0. The random variable's parameters will remain unchanged.")
        } else {
            self.mean = mean;
            self.std = std;
            Ok((self.mean, self.std))
        }
    }
}

impl core::fmt::Display for Normal {
    /// Formatter for displaying in functions like println! macro
    /// * Mean
    /// * Standard deviation
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "N(Mean, Std^2) = N({}, {}^2)", self.mean, self.std)?;
        Ok(())
    }
}
