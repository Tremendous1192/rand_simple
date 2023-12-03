use crate::create_state;
use crate::standard_distributions::xorshift160_0_open_1_open;

/// 反射ワイブル分布
/// # 使用例
/// ```
/// let mut reflected_weibull = rand_simple::ReflectedWeibull::new(1192u32);
/// assert_eq!(format!("{reflected_weibull}"), "RWeibull(Shape parameter, Location Parameter, Scale parameter) = RWeibull(1, 0, 1)");
///
/// // 確率変数のパラメータを変更する場合
/// let shape: f64 = 2f64;
/// let location: f64 = 3f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64, f64), &str> = reflected_weibull.try_set_params(shape, location, scale);
/// assert_eq!(format!("{reflected_weibull}"), "RWeibull(Shape parameter, Location Parameter, Scale parameter) = RWeibull(2, 3, 1.5)");
/// ```
pub struct ReflectedWeibull {
    xyzuv: [u32; 5], // 状態変数
    shape: f64,  // 形状母数の逆数
    location: f64,   // 位置母数
    scale: f64,      // 尺度母数
}

impl ReflectedWeibull {
    /// Constructor
    /// * `_seed` - Random seed
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            shape: 1_f64,
            location: 0_f64,
            scale: 1_f64,
        }
    }

    /// Computes a random number.
    pub fn sample(&mut self) -> f64 {
        // Algorithm 3.53: Inverse Transform Sampling
        // Step 1: Uniform random number in (0, 1)
        let u = xorshift160_0_open_1_open(&mut self.xyzuv);
        // Step 2
        if u < 0.5_f64 {
            // Apply the inverse function for the lower half of the distribution
            -(-(2_f64 * u).ln()).powf(self.shape.powi(-1)) * self.scale + self.location
        } else {
            // Apply the inverse function for the upper half of the distribution
            (-(2_f64 * (1_f64 - u)).ln()).powf(self.shape.powi(-1)) * self.scale + self.location
        }
    }

    /// Changes the parameters of the probability variable.
    /// * `shape` - Shape parameter
    /// * `location` - Location parameter
    /// * `scale` - Scale parameter
    pub fn try_set_params(
        &mut self,
        shape: f64,
        location: f64,
        scale: f64,
    ) -> Result<(f64, f64, f64), &str> {
        if shape <= 0_f64 || scale <= 0_f64 {
            // Returns an error if the shape or scale parameters are less than or equal to 0.
            Err("Shape or scale parameter is less than or equal to 0. The parameters of the probability variable will remain unchanged.")
        } else {
            // Updates the parameters and returns the new values.
            self.shape = shape;
            self.location = location;
            self.scale = scale;
            Ok((shape, location, scale))
        }
    }
}

impl std::fmt::Display for ReflectedWeibull {
    /// Formatter for displaying using println! macro and similar constructs.
    /// * Type of the struct
    /// * Shape parameter (inverse)
    /// * Location parameter
    /// * Scale parameter
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "RWeibull(Shape parameter, Location Parameter, Scale parameter) = RWeibull({}, {}, {})",
            self.shape, self.location, self.scale
        )?;
        Ok(())
    }
}
