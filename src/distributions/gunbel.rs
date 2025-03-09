use crate::standard_distributions::{generate_random_state, standard_exponential};

/// Gunbel Distribution
/// # Examples
/// ```
/// let mut gunbel = rand_simple::Gunbel::new(1192u32);
/// assert_eq!(format!("{gunbel}"), "Gu(Location parameter, Scale parameter) = Gu(0, 1)");
/// println!("Returns a random number -> {}", gunbel.sample());
///
/// // If you want to change the parameters of the random variable
/// let location: f64 = 3_f64;
/// let scale: f64 = 1.5_f64;
/// let result: Result<(f64, f64), &str> = gunbel.try_set_params(location, scale);
/// assert_eq!(format!("{gunbel}"), "Gu(Location parameter, Scale parameter) = Gu(3, 1.5)");
/// println!("Returns a random number -> {}", gunbel.sample());
/// ```
pub struct Gunbel {
    xyzuv: [u32; 5], // 状態変数
    location: f64,   // 位置母数
    scale: f64,      // 尺度母数
}

impl Gunbel {
    /// Constructor for the Gunbel distribution.
    /// # Arguments
    /// * `_seed` - Seed for the random number generator.
    /// # Examples
    /// ```
    /// let gunbel = rand_simple::Gunbel::new(1192u32);
    /// println!("Gunbel distribution initialized with seed 1192");
    /// ```
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: generate_random_state(_seed),
            location: 0_f64,
            scale: 1_f64,
        }
    }

    /// Calculate a random number from the distribution.
    /// The method used is Algorithm 3.53: Inverse Transform Sampling.
    /// # Examples
    /// ```
    /// let mut gunbel = rand_simple::Gunbel::new(1192u32);
    /// let result = gunbel.sample();
    /// println!("Random number from Gunbel distribution: {}", result);
    /// ```
    pub fn sample(&mut self) -> f64 {
        loop {
            let z = standard_exponential(&mut self.xyzuv);
            if z > 0_f64 {
                return -z.ln() * self.scale + self.location;
            }
        }
    }

    /// Try to set parameters of the probability variable.
    /// * `location` - Location parameter
    /// * `scale` - Scale parameter
    /// # Errors
    /// Returns an error if the scale parameter is less than or equal to zero.
    /// In such cases, the parameters of the probability variable are maintained from the previous setting.
    /// # Examples
    /// ```
    /// let mut gunbel = rand_simple::Gunbel::new(1192u32);
    /// let location: f64 = 2_f64;
    /// let scale: f64 = 1.5_f64;
    /// let result: Result<(f64, f64), &str> = gunbel.try_set_params(location, scale);
    /// assert_eq!(format!("{gunbel}"), "Gu(Location parameter, Scale parameter) = Gu(2, 1.5)");
    /// ```
    pub fn try_set_params(&mut self, location: f64, scale: f64) -> Result<(f64, f64), &str> {
        if scale <= 0_f64 {
            Err("Scale parameter is less than or equal to zero. The parameters are maintained from the previous setting.")
        } else {
            self.location = location;
            self.scale = scale;
            Ok((location, scale))
        }
    }
}

/// Gunbel Distribution
/// # Display
/// Formats the struct for display using macros like println!
/// * Struct type
/// * Location parameter
/// * Scale parameter
impl std::fmt::Display for Gunbel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Gu(Location parameter, Scale parameter) = Gu({}, {})",
            self.location, self.scale
        )?;
        Ok(())
    }
}
