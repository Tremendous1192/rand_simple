use crate::create_state;
use crate::standard_distributions::standard_exponential;

/// Weibull Distribution
/// # Example
/// ```
/// let mut weibull = rand_simple::Weibull::new(1192u32);
/// assert_eq!(format!("{weibull}"), "Weibull(Shape parameter, Scale parameter) = Weibull(1, 1)");
///
/// // If you want to change the parameters of the random variable
/// let shape: f64 = 2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = weibull.try_set_params(shape, scale);
/// assert_eq!(format!("{weibull}"), "Weibull(Shape parameter, Scale parameter) = Weibull(2, 1.5)");
/// ```
pub struct Weibull {
    xyzuv: [u32; 5], // 状態変数
    shape: f64,      // 形状母数
    scale: f64,      // 尺度母数
}

impl Weibull {
    /// Constructor for the Weibull random number generator.
    /// * `_seed` - Seed for the random number generator.
    pub fn new(_seed: u32) -> Self {
        // Create a new instance of the Weibull random number generator with the specified seed,
        // and default shape and scale parameters set to 1.0.
        Self {
            xyzuv: create_state(_seed),
            shape: 1_f64,
            scale: 1_f64,
        }
    }

    /// Computes a random number.
    /// Returns a random number sampled from the Weibull distribution with the specified shape and scale parameters.
    pub fn sample(&mut self) -> f64 {
        loop {
            // Generate a random number from the standard exponential distribution.
            let z = standard_exponential(&mut self.xyzuv);

            // Check if the generated number is greater than 0.
            if z > 0_f64 {
                // Calculate the random number from the Weibull distribution using the generated value.
                return z.powf(self.shape.powi(-1)) * self.scale;
            }
        }
    }

    /// Attempts to set the parameters of the probability variable.
    /// * `shape` - Shape parameter.
    /// * `scale` - Scale parameter.
    ///   Returns a Result containing a tuple (shape, scale) on success, or an error message if the parameters are invalid.
    pub fn try_set_params(&mut self, shape: f64, scale: f64) -> Result<(f64, f64), &str> {
        // Check if the shape or scale parameters are non-positive, and return an error if so.
        if shape <= 0_f64 || scale <= 0_f64 {
            Err("Shape or scale parameter is less than or equal to 0. Parameters will be maintained from the previous setting.")
        } else {
            // Set the shape and scale parameters and return a tuple (shape, scale).
            self.shape = shape;
            self.scale = scale;
            Ok((shape, scale))
        }
    }
}

// Implementation of the Display trait for the Weibull struct, allowing custom formatting when using format! or println!
impl core::fmt::Display for Weibull {
    /// Implements the Display trait for the Weibull struct.
    /// This allows the struct to be formatted and displayed using macros like println!
    /// It displays information about the scale parameter.
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Weibull(Shape parameter, Scale parameter) = Weibull({}, {})",
            self.shape, self.scale
        )?;
        Ok(())
    }
}
