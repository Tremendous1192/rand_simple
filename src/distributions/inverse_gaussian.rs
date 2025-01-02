use crate::create_state;
use crate::standard_distributions::{standard_normal, xorshift160_0_to_1};

/// Represents an Inverse Gaussian (IG) distribution.
///
/// # Example Usage
/// ```
/// // Create a new InverseGaussian instance with a seed
/// let mut inverse_gaussian = rand_simple::InverseGaussian::new([1192u32, 765u32, 1543u32]);
/// assert_eq!(format!("{inverse_gaussian}"), "IG(Mean, Shape) = IG(1, 1)");
/// println!(
///     "Generate a random number from the standard Inverse Gaussian distribution with mean μ = 1 and shape parameter λ = 1 -> {}",
///     inverse_gaussian.sample()
/// );
///
/// // Modify the distribution parameters
/// let mean: f64 = 1.5f64;
/// let shape: f64 = 2f64;
/// let result: Result<(f64, f64), &str> = inverse_gaussian.try_set_params(mean, shape);
/// assert_eq!(format!("{inverse_gaussian}"), "IG(Mean, Shape) = IG(1.5, 2)");
/// println!(
///     "Generate a random number from the Inverse Gaussian distribution with mean μ = {}, shape λ = {} -> {}",
///     mean, shape, inverse_gaussian.sample()
/// );
/// ```
pub struct InverseGaussian {
    xyzuv_u: [u32; 5],    // 状態変数
    xyzuv_hn_0: [u32; 5], // 状態変数
    xyzuv_hn_1: [u32; 5], // 状態変数
    mean: f64,            // 平均
    shape: f64,           // 形状母数
}

impl InverseGaussian {
    /// Constructs a new `InverseGaussian` instance.
    ///
    /// # Parameters
    /// - `seeds`: An array of three `u32` values used to initialize the random number generator states.
    ///   To ensure good randomness and avoid duplicate seeds, the constructor internally adjusts these values.
    ///
    /// # Notes
    /// - The `mean` parameter is initialized to `1.0`.
    /// - The `shape` parameter is initialized to `1.0`.
    /// - Internal state variables (`xyzuv_u`, `xyzuv_hn_0`, `xyzuv_hn_1`) are derived from the adjusted seeds.
    pub fn new(seeds: [u32; 3]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv_u: create_state(adjusted_seeds[0]),
            xyzuv_hn_0: create_state(adjusted_seeds[1]),
            xyzuv_hn_1: create_state(adjusted_seeds[2]),
            mean: 1_f64,
            shape: 1_f64,
        }
    }

    /// Generates a random number following the inverse Gaussian distribution.
    ///
    /// # Algorithm
    /// Implements Algorithm 3.94 for generating samples from the inverse Gaussian distribution.
    ///
    /// ## Steps
    /// 1. **Preprocessing**: Calculate intermediate values `p` and `q` based on the distribution's parameters.
    /// 2. **Generate a standard normal random variable** `z`. If `z` is zero, return the mean as the result.
    /// 3. **Compute candidate value** `x_1` based on the adjusted mean and shape parameters.
    /// 4. **Acceptance-Rejection Step**:
    ///    - Generate a uniform random variable `u`.
    ///    - Depending on the relationship between `u`, `x_1`, and the mean, decide whether to accept `x_1` or use an alternative value.
    /// 5. **Return the final value** as the sample.
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.94
        // 前処理
        let p = self.mean.powi(2);
        let q = p / (2_f64 * self.shape);

        // step 1
        let z = standard_normal(&mut self.xyzuv_hn_0, &mut self.xyzuv_hn_1).abs();
        if z == 0_f64 {
            // step 1 -> step 5
            self.mean
        } else {
            // step 2
            let v = self.mean + q * z.powi(2);
            let x_1 = v + (v.powi(2) - p).sqrt();

            // step 3
            let u = xorshift160_0_to_1(&mut self.xyzuv_u);
            if u * (x_1 + self.mean) <= self.mean {
                // step 3 -> step 5
                x_1
            } else {
                // step 4 -> step 5
                p / x_1
            }
        }
    }

    /// Updates the parameters of the inverse Gaussian random variable.
    ///
    /// # Parameters
    /// - `mean`: The mean parameter (μ) of the distribution. Must be positive.
    /// - `shape`: The shape parameter (λ) of the distribution. Must be positive.
    ///
    /// # Returns
    /// - `Ok((mean, shape))`: If both parameters are valid, the new values are set, and they are returned as a tuple.
    /// - `Err(&str)`: If either parameter is invalid, an error message is returned, and the previous parameter values are retained.
    ///
    /// # Validations
    /// - `mean > 0`: Ensures the mean is strictly positive. If this condition is violated, an error is returned.
    /// - `shape > 0`: Ensures the shape parameter is strictly positive. If this condition is violated, an error is returned.
    ///
    /// # Example
    /// ```rust
    /// let mut inverse_gaussian = rand_simple::InverseGaussian::new([1192u32, 765u32, 1543u32]);
    /// assert_eq!(format!("{inverse_gaussian}"), "IG(Mean, Shape) = IG(1, 1)");
    ///
    /// // Attempt to update with valid parameters.
    /// let result = inverse_gaussian.try_set_params(1.5, 2.0);
    /// assert!(result.is_ok());
    /// assert_eq!(format!("{inverse_gaussian}"), "IG(Mean, Shape) = IG(1.5, 2)");
    ///
    /// // Attempt to update with an invalid mean.
    /// let result = inverse_gaussian.try_set_params(-1.0, 2.0);
    /// assert!(result.is_err());
    /// println!("{}", result.unwrap_err()); // Output: "平均が0以下です..."
    /// ```
    pub fn try_set_params(&mut self, mean: f64, shape: f64) -> Result<(f64, f64), &str> {
        if mean <= 0_f64 {
            Err("Mean is less than or equal to 0. The previous parameter values are retained.")
        } else if shape <= 0_f64 {
            Err("Shape parameter is less than or equal to 0. The previous parameter values are retained.")
        } else {
            self.mean = mean;
            self.shape = shape;
            core::result::Result::Ok((mean, shape))
        }
    }
}

impl core::fmt::Display for InverseGaussian {
    /// Formatter for displaying in functions like println! macro
    /// * Mean
    /// * Standard deviation
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "IG(Mean, Shape) = IG({}, {})", self.mean, self.shape)?;
        Ok(())
    }
}
