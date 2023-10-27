use crate::create_state;
use crate::standard_distributions::standard_exponential;

/// Exponential Distribution
/// # Example
/// ```
/// let mut exponential = rand_simple::Exponential::new(1192_u32);
/// // Ensure that the distribution's representation includes the scale parameter
/// assert_eq!(format!("{exponential}"), "Exp(Scale parameter) = Exp(1)");
/// // Generate a random number following the Exponential distribution with the initial scale parameter
/// println!("Returns a random number -> {}", exponential.sample());
///
/// // Modify the distribution's scale parameter
/// let scale: f64 = 1.5_f64;
/// // Generate a random number following the modified Exponential distribution with scale parameter θ
/// let result: Result<f64, &str> = exponential.try_set_params(scale);
/// // Ensure that the distribution's representation includes the scale parameter
/// assert_eq!(format!("{exponential}"), "Exp(Scale parameter) = Exp(1.5)");
/// // Generate a random number following the Exponential distribution with the updated scale parameter
/// println!("Returns a random number -> {}", exponential.sample());
/// ```
pub struct Exponential {
    xyzuv: [u32; 5], // 状態変数
    scale: f64,      // 尺度母数
}

impl Exponential {
    /// Constructor
    /// * `_seed` - Random number seed
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            scale: 1_f64,
        }
    }

    /// Calculate random numbers.
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.41: 逆関数法
        standard_exponential(&mut self.xyzuv) * self.scale
    }

    /// Modify the parameters of the probability variable.
    /// * `scale` - Scale parameter
    pub fn try_set_params(&mut self, scale: f64) -> Result<f64, &'static str> {
        if scale <= 0_f64 {
            Err("The scale parameter is less than or equal to 0. The parameters of the probability variable will remain unchanged.")
        } else {
            self.scale = scale;
            Ok(scale)
        }
    }
}

impl core::fmt::Display for Exponential {
    /// Formatter for displaying with macros like println!
    /// * Scale parameter
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Exp(Scale parameter) = Exp({})", self.scale)?;
        Ok(())
    }
}
