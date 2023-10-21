use crate::standard_distributions::xorshift160_0_1;
use crate::{create_state, Uniform};

impl Uniform {
    /// Constructor
    /// * `_seed` - The seed for the random number generator
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            min: 0_f64,
            max: 1_f64,
        }
    }

    /// Calculate a random number.
    /// # Returns
    /// A random floating-point number within the specified range.
    pub fn sample(&mut self) -> f64 {
        xorshift160_0_1(&mut self.xyzuv) * (self.max - self.min) + self.min
    }

    /// Attempt to modify the parameters of the random variable.
    /// * `min` - Minimum value
    /// * `max` - Maximum value
    pub fn try_set_params(&mut self, min: f64, max: f64) -> Result<(f64, f64), &'static str> {
        if min >= max {
            Err("The minimum and maximum values are equal or the minimum value is greater. The parameters of the random variable will remain unchanged.")
        } else {
            self.min = min;
            self.max = max;
            Ok((self.min, self.max))
        }
    }
}

impl core::fmt::Display for Uniform {
    /// Formatter for displaying with println! macro and others.
    /// * Range (Closed Interval)
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Range (Closed Interval): [{}, {}]", self.min, self.max)?;
        Ok(())
    }
}
