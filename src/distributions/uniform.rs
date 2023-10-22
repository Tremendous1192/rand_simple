use crate::create_state;
use crate::standard_distributions::xorshift160_0_1;

/// Uniform Distribution
/// # Example
/// ```
/// let mut uniform = rand_simple::Uniform::new(1192_u32);
/// assert_eq!(format!("{uniform}"), "Range (Closed Interval): [0, 1]");
/// println!("Returns a random number -> {}", uniform.sample());
///
/// // When changing the parameters of the random variable
/// let min: f64 = -1_f64;
/// let max: f64 = 1_f64;
/// let result: Result<(f64, f64), &str> = uniform.try_set_params(min, max);
/// assert_eq!(format!("{uniform}"), "Range (Closed Interval): [-1, 1]");
/// println!("Returns a random number -> {}", uniform.sample());
/// ```
pub struct Uniform {
    xyzuv: [u32; 5], // 状態変数
    min: f64,        // 最小値
    max: f64,        // 最大値
}

impl Uniform {
    /// Constructor
    /// * `_seed` - The seed for the random number generator
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            min: 0_f64,
            max: 1_f64,
        }
    }

    /// Calculate a random number.
    /// # Returns
    /// A random floating-point number within the specified range.
    pub fn sample(&mut self) -> f64 {
        xorshift160_0_1(&mut self.xyzuv) * (self.max - self.min) + self.min
    }

    /// Attempt to modify the parameters of the random variable.
    /// * `min` - Minimum value
    /// * `max` - Maximum value
    pub fn try_set_params(&mut self, min: f64, max: f64) -> Result<(f64, f64), &'static str> {
        if min >= max {
            Err("The minimum and maximum values are equal or the minimum value is greater. The parameters of the random variable will remain unchanged.")
        } else {
            self.min = min;
            self.max = max;
            Ok((self.min, self.max))
        }
    }
}

impl core::fmt::Display for Uniform {
    /// Formatter for displaying with println! macro and others.
    /// * Range (Closed Interval)
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Range (Closed Interval): [{}, {}]", self.min, self.max)?;
        Ok(())
    }
}
