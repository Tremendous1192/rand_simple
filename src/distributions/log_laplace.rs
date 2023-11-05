use crate::create_state;
use crate::standard_distributions::standard_laplace;

/// Log-Laplace Distribution
/// # Example
/// ```
/// let mut log_laplace = rand_simple::LogLaplace::new(1192u32);
/// assert_eq!(format!("{log_laplace}"), "LLa(Location parameter, Scale parameter) = LLa(0, 1)");
/// println!("Returns a random number -> {}", log_laplace.sample());
///
/// // Modify the distribution's parameters
/// let location: f64 = -2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = log_laplace.try_set_params(location, scale);
/// assert_eq!(format!("{log_laplace}"), "LLa(Location parameter, Scale parameter) = LLa(-2, 1.5)");
/// println!("Returns a random number -> {}", log_laplace.sample());
/// ```
pub struct LogLaplace {
    xyzuv: [u32; 5], // 状態変数
    location: f64,   // 位置母数
    scale: f64,      // 尺度母数
}

impl LogLaplace {
    /// Constructor
    /// * `_seed` - Random seed to initialize the generator.
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            location: 0_f64,
            scale: 1_f64,
        }
    }

    /// Calculate a random number.
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.49
        (standard_laplace(&mut self.xyzuv) * self.scale + self.location).exp()
    }

    /// Modify the parameters of the random variable.
    /// * `location` - Location parameter
    /// * `scale` - Scale parameter
    pub fn try_set_params(
        &mut self,
        location: f64,
        scale: f64,
    ) -> Result<(f64, f64), &'static str> {
        if scale <= 0_f64 {
            Err("The scale parameter is less than or equal to 0. The parameters of the random variable will remain as previously set.")
        } else {
            self.location = location;
            self.scale = scale;
            Ok((location, scale))
        }
    }
}

/// Formatter for displaying with macros like println!
/// * Location parameter
/// * Scale parameter
impl core::fmt::Display for LogLaplace {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "LLa(Location parameter, Scale parameter) = LLa({}, {})",
            self.location, self.scale
        )?;
        Ok(())
    }
}
