use crate::create_state;
use crate::standard_distributions::standard_laplace;

/// Laplace Distribution
/// # Example
/// ```
/// let mut laplace = rand_simple::Laplace::new(1192u32);
/// assert_eq!(format!("{laplace}"), "La(Location parameter, Scale parameter) = La(0, 1)");
/// println!("Returns a random number -> {}", laplace.sample());
///
/// // Modify the distribution's parameters
/// let location: f64 = -2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = laplace.try_set_params(location, scale);
/// assert_eq!(format!("{laplace}"), "La(Location parameter, Scale parameter) = La(-2, 1.5)");
/// println!("Returns a random number -> {}", laplace.sample());
/// ```
pub struct Laplace {
    xyzuv: [u32; 5], // 状態変数
    location: f64,   // 位置母数
    scale: f64,      // 尺度母数
}

impl Laplace {
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
        standard_laplace(&mut self.xyzuv) * self.scale + self.location
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
impl std::fmt::Display for Laplace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "La(Location parameter, Scale parameter) = La({}, {})",
            self.location, self.scale
        )?;
        Ok(())
    }
}
