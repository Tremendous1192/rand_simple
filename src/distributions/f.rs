use crate::standard_distributions::{
    generate_random_state, standard_gamma, xorshift160_0_or_greater_and_less_than_1,
};

/// F-distribution
/// # Usage Example
/// ```
/// // Create a new FDistribution instance with the provided seeds.
/// // These seeds are used to initialize the internal state variables necessary for the random number generation.
/// let mut f = rand_simple::FDistribution::new([1192_u32, 765_u32, 1543_u32, 2003_u32, 1192_u32, 765_u32, 1543_u32, 2003_u32]);
///
/// // Check that the initial degrees of freedom parameters are correctly set to (1, 1).
/// // This ensures that the F-distribution is initialized with the default settings.
/// assert_eq!(format!("{f}"), "F(Degree of Freedom parameter 1, Degree of Freedom parameter 2) = F(1, 1)");
///
/// // Generate and print a random number from the F-distribution with the initial degrees of freedom (1, 1).
/// // The `sample()` method is used to produce this random number.
/// println!("With the initial setup, generates a random number following the F-distribution with degrees of freedom (1, 1) -> {}", f.sample());
///
/// // Modify the degrees of freedom parameters.
/// // Here, `degree_of_freedom_1` is set to 2 and `degree_of_freedom_2` is set to 3, changing the shape of the distribution.
/// let degree_of_freedom_1: u64 = 2_u64;  // New degrees of freedom for the first parameter.
/// let degree_of_freedom_2: u64 = 3_u64;  // New degrees of freedom for the second parameter.
///
/// // Attempt to update the F-distribution's parameters using the new degrees of freedom.
/// // The `try_set_params` method returns a `Result`, allowing us to handle any potential errors during this update.
/// let result: Result<(u64, u64), &str> = f.try_set_params(degree_of_freedom_1, degree_of_freedom_2);
///
/// // Verify that the degrees of freedom have been successfully updated to (2, 3).
/// // This assertion ensures that the distribution is now set up with the new parameters.
/// assert_eq!(format!("{f}"), "F(Degree of Freedom parameter 1, Degree of Freedom parameter 2) = F(2, 3)");
///
/// // Generate and print a random number from the F-distribution with the updated degrees of freedom (2, 3).
/// // The `sample()` method now reflects the updated distribution parameters.
/// println!("Generates a random number with the updated degrees of freedom ({}, {}) -> {}", degree_of_freedom_1, degree_of_freedom_2, f.sample());
/// ```

pub struct FDistribution {
    xyzuv_u_gamma_1: [u32; 5],   // 状態変数
    xyzuv_n_0_gamma_1: [u32; 5], // 状態変数
    xyzuv_n_1_gamma_1: [u32; 5], // 状態変数

    xyzuv_uniform_1: [u32; 5], // 状態変数

    degree_of_freedom_1: u64, // 自由度 r ∈ N

    xyzuv_u_gamma_2: [u32; 5],   // 状態変数
    xyzuv_n_0_gamma_2: [u32; 5], // 状態変数
    xyzuv_n_1_gamma_2: [u32; 5], // 状態変数

    xyzuv_uniform_2: [u32; 5], // 状態変数

    degree_of_freedom_2: u64, // 自由度 r ∈ N
}

impl FDistribution {
    /// Constructor for the `FDistribution` struct.
    /// This function initializes a new instance of the `FDistribution` with given seeds.
    ///
    /// # Arguments
    ///
    /// * `seeds` - An array of 8 unsigned 32-bit integers (`u32`). These serve as the seeds for the random number generators
    ///   that will be used within the distribution. The seeds are critical for ensuring the randomness of the generated numbers.
    ///
    /// # Returns
    ///
    /// Returns an instance of the `FDistribution` struct initialized with the provided seeds.
    pub fn new(seeds: [u32; 8_usize]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);

        Self {
            xyzuv_u_gamma_1: generate_random_state(adjusted_seeds[0]),
            xyzuv_n_0_gamma_1: generate_random_state(adjusted_seeds[1]),
            xyzuv_n_1_gamma_1: generate_random_state(adjusted_seeds[2]),

            xyzuv_uniform_1: generate_random_state(adjusted_seeds[3]),

            degree_of_freedom_1: 1_u64,

            xyzuv_u_gamma_2: generate_random_state(adjusted_seeds[4]),
            xyzuv_n_0_gamma_2: generate_random_state(adjusted_seeds[5]),
            xyzuv_n_1_gamma_2: generate_random_state(adjusted_seeds[6]),

            xyzuv_uniform_2: generate_random_state(adjusted_seeds[7]),

            degree_of_freedom_2: 1_u64,
        }
    }

    /// Generates a random number that follows the F-distribution.
    ///
    /// This method implements the algorithm to compute a random variable that follows the F-distribution
    /// by generating two chi-squared distributed random variables and then computing their ratio.
    ///
    /// # Returns
    ///
    /// Returns a `f64` value that represents a random sample from the F-distribution.
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.83
        // step 1: χ二乗分布の確率変数を2個生成する
        let chi_1 = if self.degree_of_freedom_1 > 1_u64 {
            standard_gamma(
                &mut self.xyzuv_u_gamma_1,
                &mut self.xyzuv_n_0_gamma_1,
                &mut self.xyzuv_n_1_gamma_1,
                &(self.degree_of_freedom_1 as f64),
            ) * 2_f64
        } else {
            let y = standard_gamma(
                &mut self.xyzuv_u_gamma_1,
                &mut self.xyzuv_n_0_gamma_1,
                &mut self.xyzuv_n_1_gamma_1,
                &(3_f64 / 2_f64),
            ) * 2_f64;
            let u = xorshift160_0_or_greater_and_less_than_1(&mut self.xyzuv_uniform_1);
            u.powi(2) * y * 2_f64
        };

        let chi_2 = if self.degree_of_freedom_2 > 1_u64 {
            standard_gamma(
                &mut self.xyzuv_u_gamma_2,
                &mut self.xyzuv_n_0_gamma_2,
                &mut self.xyzuv_n_1_gamma_2,
                &(self.degree_of_freedom_2 as f64),
            ) * 2_f64
        } else {
            let y = standard_gamma(
                &mut self.xyzuv_u_gamma_2,
                &mut self.xyzuv_n_0_gamma_2,
                &mut self.xyzuv_n_1_gamma_2,
                &(3_f64 / 2_f64),
            ) * 2_f64;
            let u = xorshift160_0_or_greater_and_less_than_1(&mut self.xyzuv_uniform_2);
            u.powi(2) * y * 2_f64
        };

        // step 2: 確率変数を計算する
        (self.degree_of_freedom_2 as f64 * chi_1) / (chi_2 * self.degree_of_freedom_1 as f64)
    }

    /// Attempts to set new parameters for the F-distribution's degrees of freedom.
    /// This function validates the provided degrees of freedom and updates the struct's internal state if the validation passes.
    /// If the validation fails, it returns an error message, and the parameters remain unchanged.
    ///
    /// # Arguments
    ///
    /// * `degree_of_freedom_1` - The first degree of freedom, associated with the numerator in the F-distribution.
    /// * `degree_of_freedom_2` - The second degree of freedom, associated with the denominator in the F-distribution.
    ///
    /// # Returns
    ///
    /// A `Result` type:
    ///
    /// * `Ok((u64, u64))` - Returns a tuple containing the updated degrees of freedom if the parameters are valid.
    /// * `Err(&str)` - Returns an error message if either degree of freedom is invalid (i.e., less than 1).
    pub fn try_set_params(
        &mut self,
        degree_of_freedom_1: u64,
        degree_of_freedom_2: u64,
    ) -> Result<(u64, u64), &str> {
        // Validate that both degrees of freedom are natural numbers (greater than or equal to 1).
        if degree_of_freedom_1 < 1_u64 || degree_of_freedom_2 < 1_u64 {
            // If either degree of freedom is invalid, return an error message.
            Err("Degrees of freedom must be natural numbers. The parameters will remain unchanged.")
        } else {
            // If the degrees of freedom are valid, update the struct's internal state.
            self.degree_of_freedom_1 = degree_of_freedom_1;
            self.degree_of_freedom_2 = degree_of_freedom_2;
            // Return the updated degrees of freedom as a tuple.
            Ok((degree_of_freedom_1, degree_of_freedom_2))
        }
    }
}

/// Implementing the `Display` trait for the `FDistribution` struct
/// This allows instances of `FDistribution` to be formatted as a string,
/// making it easier to print and debug the object.
impl core::fmt::Display for FDistribution {
    /// Formats the `FDistribution` instance for display purposes.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a `core::fmt::Formatter` that handles the formatting output.
    ///
    /// # Returns
    ///
    /// This function returns a `core::fmt::Result`, indicating whether the formatting was successful.
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Writes the formatted string into the formatter `f`.
        // The format string "F(Degree of Freedom parameter 1, Degree of Freedom parameter 2) = F({}, {})"
        // will display the two degrees of freedom stored in the struct.
        write!(
            f,
            "F(Degree of Freedom parameter 1, Degree of Freedom parameter 2) = F({}, {})",
            self.degree_of_freedom_1, self.degree_of_freedom_2
        )?;
        // Returns an Ok result, signaling that the formatting was completed successfully.
        Ok(())
    }
}
