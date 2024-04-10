use crate::standard_distributions::standard_gamma;
use crate::{create_state, Erlang};

impl Erlang {
    /// Constructor
    ///
    /// Creates a new instance of the Erlang distribution with the provided seeds for random number generation.
    /// Seeds are adjusted internally to ensure uniqueness.
    ///
    /// # Arguments
    ///
    /// * `seeds` - Seeds for random number generation. Adjusted internally to ensure uniqueness.
    ///
    /// # Returns
    ///
    /// A new instance of the Erlang distribution.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new Erlang distribution with specified seeds
    /// let mut erlang = rand_simple::Erlang::new([1192u32, 765u32, 1543u32]);
    /// ```
    pub fn new(seeds: [u32; 3_usize]) -> Self {
        // Adjust seeds to ensure uniqueness
        let adjusted_seeds = crate::adjust_seeds!(seeds);

        // Create a new instance with adjusted seeds and default parameters
        Self {
            xyzuv_u: create_state(adjusted_seeds[0]),
            xyzuv_n_0: create_state(adjusted_seeds[1]),
            xyzuv_n_1: create_state(adjusted_seeds[2]),
            shape: 1_f64, // Default shape parameter
            scale: 1_f64, // Default scale parameter
        }
    }

    /// Generate a random number.
    ///
    /// This method calculates a random number using the standard gamma distribution.
    ///
    /// # Returns
    ///
    /// Returns a random number generated from the standard gamma distribution, multiplied by the scale parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut erlang = rand_simple::Erlang::new([1192u32, 765u32, 1543u32]);
    /// let random_number = erlang.sample();
    /// println!("Random number: {}", random_number);
    /// ```
    pub fn sample(&mut self) -> f64 {
        // Call the standard_gamma function to generate a random number
        standard_gamma(
            &mut self.xyzuv_u,   // Mutable reference to the state variable
            &mut self.xyzuv_n_0, // Mutable reference to the state variable
            &mut self.xyzuv_n_1, // Mutable reference to the state variable
            &self.shape,         // Shape parameter of the gamma distribution
        ) * self.scale // Multiply the generated random number by the scale parameter
    }

    /// Attempt to modify the parameters of the random variable.
    /// * `shape` - Shape parameter.
    /// * `scale` - Scale parameter.
    ///
    /// # Arguments
    ///
    /// * `shape` - Shape parameter.
    /// * `scale` - Scale parameter.
    ///
    /// # Returns
    ///
    /// Returns a tuple `(i64, f64)` representing the modified parameters `(shape, scale)`.
    ///
    /// # Errors
    ///
    /// Returns an error message if the provided parameters are invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut erlang = rand_simple::Erlang::new([1192u32, 765u32, 1543u32]);
    /// assert_eq!(erlang.try_set_params(3, 0.5), Ok((3, 0.5)));
    /// assert_eq!(erlang.try_set_params(0, 0.5), Err("The shape parameter is less than or equal to 0. The parameters of the random variable will remain unchanged."));
    /// assert_eq!(erlang.try_set_params(1, -0.1), Err("The scale parameter is less than or equal to 0. The parameters of the random variable will remain unchanged."));
    /// ```
    pub fn try_set_params(&mut self, shape: i64, scale: f64) -> Result<(i64, f64), &str> {
        if shape <= 0_i64 {
            Err("The shape parameter is less than or equal to 0. The parameters of the random variable will remain unchanged.")
        } else if shape as f64 == 1_f64 / 3_f64 {
            Err("The shape parameter is equal to 1/3. The parameters of the random variable will remain unchanged.")
        } else if scale <= 0_f64 {
            Err("The scale parameter is less than or equal to 0. The parameters of the random variable will remain unchanged.")
        } else {
            self.shape = shape as f64;
            self.scale = scale;
            Ok((shape, scale))
        }
    }
}

impl core::fmt::Display for Erlang {
    /// Formatter for displaying in macros like print!
    /// * Shape parameter
    /// * Scale parameter
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Er(Shape parameter, Scale parameter) = Er({}, {})",
            self.shape, self.scale
        )?;
        Ok(())
    }
}
