use crate::create_state;
use crate::standard_distributions::standard_cauchy;

/// Cauchy Distribution
/// # Example
/// ```
/// let mut cauchy = rand_simple::Cauchy::new(1192_u32);
/// assert_eq!(format!("{cauchy}"), "Ca(Location parameter, Scale parameter) = Ca(0, 1)");
/// println!("Returns a random number -> {}", cauchy.sample());
///
/// // When changing the parameters of the random variable
/// let location: f64 = -2_f64;
/// let scale: f64 = 1.5_f64;
/// let result: Result<(f64, f64), &str> = cauchy.try_set_params(location, scale);
/// assert_eq!(format!("{cauchy}"), "Ca(Location parameter, Scale parameter) = Ca(-2, 1.5)");
/// println!("Returns a random number -> {}", cauchy.sample());
/// ```
pub struct Cauchy {
    xyzuv: [u32; 5], // 状態変数
    location: f64,   // 位置母数
    scale: f64,      // 尺度母数
}

impl Cauchy {
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
        standard_cauchy(&mut self.xyzuv) * self.scale + self.location
    }

    /// Modify the parameters of the random variable.
    /// * `location` - Location parameter
    /// * `scale` - Scale parameter
    pub fn try_set_params(&mut self, location: f64, scale: f64) -> Result<(f64, f64), &'static str> {
        if scale <= 0_f64 {
            Err("The scale parameter is less than or equal to 0. The parameters of the random variable will remain as previously set.")
        } else {
            self.location = location;
            self.scale = scale;
            Ok((location, scale))
        }
    }
}

impl core::fmt::Display for Cauchy {
    /// Formatter for displaying in macros like print!
    /// * Location parameter
    /// * Scale parameter
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Ca(Location parameter, Scale parameter) = Ca({}, {})",
            self.location, self.scale
        )?;
        Ok(())
    }
}
