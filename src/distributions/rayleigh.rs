use crate::standard_distributions::{generate_random_state, standard_exponential};

/// Rayleigh distribution.
/// # Examples
/// ```
/// let mut rayleigh = rand_simple::Rayleigh::new(1192u32);
/// assert_eq!(format!("{rayleigh}"), "Rayleigh(Scale parameter) = Rayleigh(1)");
///
/// // To change the parameter of the random variable
/// let scale: f64 = 1.5f64;
/// let result: Result<f64, &str> = rayleigh.try_set_params(scale);
/// assert_eq!(format!("{rayleigh}"), "Rayleigh(Scale parameter) = Rayleigh(1.5)");
/// ```
pub struct Rayleigh {
    xyzuv: [u32; 5], // 状態変数
    scale: f64,      // 尺度母数
}

impl Rayleigh {
    /// Constructor for initializing a new random number generator.
    /// * `_seed` - The seed for the random number generator.
    pub fn new(_seed: u32) -> Self {
        // Create a new instance of the random number generator with the specified seed.
        Self {
            xyzuv: generate_random_state(_seed),
            scale: 1_f64,
        }
    }

    /// Generate a random sample.
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.51
        (2f64 * standard_exponential(&mut self.xyzuv)).sqrt() * self.scale
    }

    /// Modify the parameters of the probability variable.
    /// * `scale` - Scale parameter
    pub fn try_set_params(&mut self, scale: f64) -> Result<f64, &str> {
        if scale <= 0_f64 {
            Err("The scale parameter is less than or equal to zero. The parameters of the probability variable will remain unchanged.")
        } else {
            self.scale = scale;
            Ok(self.scale)
        }
    }
}

impl core::fmt::Display for Rayleigh {
    /// Implements the Display trait for the Rayleigh struct.
    /// This allows the struct to be formatted and displayed using macros like println!
    /// It displays information about the scale parameter.
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Rayleigh(Scale parameter) = Rayleigh({})", self.scale)?;
        Ok(())
    }
}
