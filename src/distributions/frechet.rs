use crate::create_state;
use crate::standard_distributions::standard_exponential;

/// Frechet Distribution
/// # Example
/// ```
/// let mut frechet = rand_simple::Frechet::new(1192u32);
/// assert_eq!(format!("{frechet}"), "Fr(Shape parameter, Scale parameter) = Fr(1, 1)");
///
/// // Modifying the parameters of the random variable
/// let shape: f64 = 2_f64;
/// let scale: f64 = 1.5_f64;
/// let result: Result<(f64, f64), &str> = frechet.try_set_params(shape, scale);
/// assert_eq!(format!("{frechet}"), "Fr(Shape parameter, Scale parameter) = Fr(2, 1.5)");
/// ```
pub struct Frechet {
    xyzuv: [u32; 5], // 状態変数
    shape: f64,      // 形状母数
    scale: f64,      // 尺度母数
}

impl Frechet {
    /// Constructor for creating a new instance of the Frechet distribution.
    ///
    /// # Arguments
    ///
    /// * `_seed` - The seed for initializing the random number generator.
    ///
    /// # Returns
    ///
    /// A new instance of the Frechet distribution with default parameters (shape = 1, scale = 1).
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            shape: 1_f64,
            scale: 1_f64,
        }
    }

    /// Generate a random number.
    pub fn sample(&mut self) -> f64 {
        loop {
            let z = standard_exponential(&mut self.xyzuv);
            if z > 0_f64 {
                // Applying the transformation to get a sample from the Fréchet distribution
                return z.powf(-self.shape.powi(-1_i32)) * self.scale;
            }
        }
    }

    /// Try to set the parameters of the random variable.
    /// * `shape` - Shape parameter.
    /// * `scale` - Scale parameter.
    pub fn try_set_params(&mut self, shape: f64, scale: f64) -> Result<(f64, f64), &str> {
        if shape <= 0_f64 || scale <= 0_f64 {
            Err("Shape or scale parameter is less than or equal to 0. Keeping the previous parameters.")
        } else {
            self.shape = shape;
            self.scale = scale;
            Ok((shape, scale))
        }
    }
}

impl core::fmt::Display for Frechet {
    /// Formatter for displaying the Frechet distribution parameters.
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Fr(Shape parameter, Scale parameter) = Fr({}, {})",
            self.shape, self.scale
        )?;
        Ok(())
    }
}
