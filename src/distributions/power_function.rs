use crate::create_state;
use crate::standard_distributions::xorshift160_0_open_1_open;

/// Power function distribution
/// # Examples
/// ```
/// let mut power_function = rand_simple::PowerFunction::new(1192u32);
/// assert_eq!(format!("{power_function}"), "PF(Shape parameter γ, Boundary parameter a, Boundary parameter b) = PF(1, 0, 1)");
///
/// // If you want to change the parameters of the random variable
/// let shape: f64 = 2_f64;
/// let min: f64 = -1_f64;
/// let max: f64 = 1_f64;
/// let result: Result<(f64, f64, f64), &str> = power_function.try_set_params(shape, min, max);
/// assert_eq!(format!("{power_function}"), "PF(Shape parameter γ, Boundary parameter a, Boundary parameter b) = PF(2, -1, 1)");
/// ```
pub struct PowerFunction {
    xyzuv: [u32; 5], // 状態変数
    shape: f64,      // 形状母数
    min_a: f64,      // 境界母数(小範)
    max_b: f64,      // 境界母数(大範)
}

impl PowerFunction {
    /// Constructor
    /// * `_seed` - Random seed
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            shape: 1_f64,
            min_a: 0_f64,
            max_b: 1_f64,
        }
    }

    /// Computes a random number
    pub fn sample(&mut self) -> f64 {
        // Algorithm 3.67
        // Step 1
        // Generate a uniform random number U in the interval (0, 1) and calculate Y = U^γ_inv
        // Step 2
        // Calculate X = a + (b - a)Y as the desired random number
        xorshift160_0_open_1_open(&mut self.xyzuv).powf(self.shape.powi(-1))
            * (self.max_b - self.min_a)
            + self.min_a
    }

    /// Changes the parameters of the probability variable
    /// * `shape` - Shape parameter γ
    /// * `min_a` - Lower boundary parameter a
    /// * `max_b` - Upper boundary parameter b
    pub fn try_set_params(
        &mut self,
        shape: f64,
        min_a: f64,
        max_b: f64,
    ) -> Result<(f64, f64, f64), &str> {
        if shape <= 0_f64 {
            Err("The shape parameter γ is less than or equal to 0. The parameters of the probability variable will remain unchanged.")
        } else if min_a >= max_b {
            Err("The upper boundary parameter b is equal to or greater than the lower boundary parameter a. The parameters of the probability variable will remain unchanged.")
        } else {
            self.shape = shape;
            self.min_a = min_a;
            self.max_b = max_b;
            Ok((shape, min_a, max_b))
        }
    }
}

impl core::fmt::Display for PowerFunction {
    /// Formatter for displaying using println! macro
    /// * Type of the structure
    /// * Scale parameter
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "PF(Shape parameter γ, Boundary parameter a, Boundary parameter b) = PF({}, {}, {})",
            self.shape, self.min_a, self.max_b
        )?;
        Ok(())
    }
}
