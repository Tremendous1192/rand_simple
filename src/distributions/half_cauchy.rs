use crate::create_state;
use crate::standard_distributions::xorshift160_0_or_greater_and_less_than_1;

/// Half Cauchy Distribution
///
/// # Example
/// ```
/// // Create a new Half-Cauchy distribution with a scale parameter of 1
/// let mut half_cauchy = rand_simple::HalfCauchy::new(1192_u32);
/// // Ensure that the distribution's representation includes the scale parameter
/// assert_eq!(format!("{half_cauchy}"), "HCa(Scale parameter) = HCa(1)");
/// // Generate a random number following the Half-Cauchy distribution with the initial scale parameter
/// println!("Returns a random number -> {}", half_cauchy.sample());
///
/// // Modify the distribution's scale parameter
/// let scale: f64 = 1.5;
/// // Update the scale parameter and generate a random number following the modified Half-Cauchy distribution
/// let result: Result<f64, &str> = half_cauchy.try_set_params(scale);
/// // Ensure that the distribution's representation includes the updated scale parameter
/// assert_eq!(format!("{half_cauchy}"), "HCa(Scale parameter) = HCa(1.5)");
/// println!("Returns a random number -> {}", half_cauchy.sample());
/// ```
pub struct HalfCauchy {
    xyzuv: [u32; 5], // 状態変数
    scale: f64,      // 尺度母数
}

impl HalfCauchy {
    /// Constructor
    ///
    /// Create a new instance of the HalfCauchy distribution with a specific random seed.
    /// The random seed ensures that different instances are not initialized with the same values.
    ///
    /// # Arguments
    /// * `_seed` - A random seed value for initializing the generator.
    ///
    /// # Returns
    /// A new instance of the HalfCauchy distribution.
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            scale: 1_f64,
        }
    }

    /// Calculate a random number.
    ///
    /// This function generates a random number following the Cauchy distribution.
    /// It uses the Marsaglia polar method to obtain a random number with a Cauchy distribution.
    /// The generated random number is scaled by the `scale` parameter.
    ///
    /// # Returns
    /// * `f64` - A random number following the Cauchy distribution scaled by the `scale` parameter.
    pub fn sample(&mut self) -> f64 {
        (std::f64::consts::PI * xorshift160_0_or_greater_and_less_than_1(&mut self.xyzuv) / 2_f64).tan() * self.scale
    }

    /// Attempt to change the parameters of the random variable.
    /// * `scale` - Scale parameter
    ///
    /// If the provided `scale` is less than or equal to zero, an error is returned.
    /// In such cases, the parameters of the random variable remain unchanged.
    ///
    /// # Arguments
    /// * `scale` - The new scale parameter to set for the random variable.
    ///
    /// # Returns
    /// * `Result<f64, &str>` - A `Result` indicating success with the new scale value or an error message.
    pub fn try_set_params(&mut self, scale: f64) -> Result<f64, &'static str> {
        if scale <= 0_f64 {
            Err("The scale parameter is less than or equal to zero. The parameters of the random variable remain unchanged.")
        } else {
            self.scale = scale;
            Ok(scale)
        }
    }
}

impl core::fmt::Display for HalfCauchy {
    /// Formatter for displaying in println! and similar macros.
    /// * Scale parameter
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "HCa(Scale parameter) = HCa({})", self.scale)?;
        Ok(())
    }
}
