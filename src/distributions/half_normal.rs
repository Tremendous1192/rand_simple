use crate::create_state;
use crate::standard_distributions::standard_normal;

/// Half Normal distribution
/// # Example
/// ```
/// let mut half_normal = rand_simple::HalfNormal::new([1192_u32, 765_u32]);
/// assert_eq!(format!("{half_normal}"), "N(Std^2) = N(1^2)");
/// println!("Returns a random number -> {}", half_normal.sample());
///
/// // If you want to change the parameters of the random variable
/// let variance: f64 = 2_f64;
/// let result: Result<f64, &str> = half_normal.try_set_params(variance);
/// assert_eq!(format!("{half_normal}"), "N(Std^2) = N(2^2)");
/// println!("Returns a random number -> {}", half_normal.sample());
/// ```
pub struct HalfNormal {
    xyzuv0: [u32; 5], // 状態変数
    xyzuv1: [u32; 5], // 状態変数
    std: f64,         // 標準偏差
}

impl HalfNormal {
    /// Constructor
    /// * `seeds` - Seeds for random number generation. Adjusted on the constructor side to ensure they are not the same.
    pub fn new(seeds: [u32; 2]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv0: create_state(adjusted_seeds[0]),
            xyzuv1: create_state(adjusted_seeds[1]),
            std: 1_f64,
        }
    }

    /// Generate a random number.
    pub fn sample(&mut self) -> f64 {
        standard_normal(&mut self.xyzuv0, &mut self.xyzuv1).abs() * self.std
    }

    /// Modify the parameters of the random variable.
    /// * `std` - Standard deviation
    pub fn try_set_params(&mut self, std: f64) -> Result<f64, &str> {
        if std <= 0_f64 {
            Err("Standard deviation is less than or equal to 0. The random variable's parameters will remain unchanged.")
        } else {
            self.std = std;
            Ok(std)
        }
    }
}

impl core::fmt::Display for HalfNormal {
    /// Formatter for displaying in functions like println! macro
    /// * Standard deviation
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "N(Std^2) = N({}^2)", self.std)?;
        Ok(())
    }
}
