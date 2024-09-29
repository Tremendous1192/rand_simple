use crate::create_state;
use crate::standard_distributions::{
    standard_cauchy, standard_exponential, standard_gamma, standard_normal,
};

/// t-distribution (Student's t-distribution)
/// # Usage Example
/// ```
/// // Create a new TDistribution instance with the provided seeds.
/// let mut t = rand_simple::TDistribution::new([1192u32, 765u32, 1543u32, 2003u32, 1867u32]);
/// // Check if the default degree of freedom is correctly set to 1.
/// assert_eq!(format!("{t}"), "T(Degree of Freedom parameter) = T(1)");
/// // Generate and print a random number based on the t-distribution with 1 degree of freedom.
/// println!("Generates a random number with degree of freedom 1 -> {}", t.sample());
///
/// // If you need to modify the parameters of the t-distribution (degree of freedom).
/// let degree_of_freedom: u64 = 3_u64;
/// // Set the new degree of freedom and check if the operation was successful.
/// let result: Result<u64, &str> = t.try_set_params(degree_of_freedom);
/// // Verify that the degree of freedom has been correctly updated to 3.
/// assert_eq!(format!("{t}"), "T(Degree of Freedom parameter) = T(3)");
/// // Generate and print a random number with the updated degree of freedom (3).
/// println!("Generates a random number with degree of freedom {} -> {}", degree_of_freedom, t.sample());
/// ```
///
/// This example demonstrates how to initialize a t-distribution instance, change the degree of freedom,
/// and generate random numbers following the t-distribution.
pub struct TDistribution {
    xyzuv_n_0: [u32; 5], // 状態変数
    xyzuv_n_1: [u32; 5], // 状態変数

    xyzuv_u_gamma: [u32; 5],   // 状態変数
    xyzuv_n_0_gamma: [u32; 5], // 状態変数
    xyzuv_n_1_gamma: [u32; 5], // 状態変数

    degree_of_freedom: u64, // 自由度 r ∈ N
}

impl TDistribution {
    /// Constructor for the `TDistribution` struct.
    /// This method initializes a new instance of the t-distribution with the provided random seeds.
    ///
    /// # Arguments
    /// * `seeds` - An array of 5 unsigned 32-bit integers (u32) that serve as seeds for generating the random numbers.
    ///
    /// The constructor processes the seeds through an adjustment function (`adjust_seeds!`)
    /// and assigns them to internal state variables used for generating random numbers
    /// that follow a t-distribution.
    ///
    /// The degree of freedom (DoF) is initialized to 1, but can be changed later via other methods.
    ///
    /// # Returns
    /// A new instance of the `TDistribution` struct.
    pub fn new(seeds: [u32; 5_usize]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);

        Self {
            xyzuv_n_0: create_state(adjusted_seeds[0]),
            xyzuv_n_1: create_state(adjusted_seeds[1]),

            xyzuv_u_gamma: create_state(adjusted_seeds[2]),
            xyzuv_n_0_gamma: create_state(adjusted_seeds[3]),
            xyzuv_n_1_gamma: create_state(adjusted_seeds[4]),

            degree_of_freedom: 1_u64,
        }
    }

    /// Generates a random sample from the t-distribution based on the current degree of freedom (DoF).
    ///
    /// # Algorithm Description:
    /// The function generates a random number following a t-distribution using different algorithms
    /// depending on the value of the degree of freedom (DoF):
    ///
    /// - For `DoF = 1`: It generates a sample from the standard Cauchy distribution.
    /// - For `DoF = 2`: It follows Algorithm 3.84, which uses a combination of normal and exponential
    ///   distributions to generate the sample.
    /// - For `DoF > 2`: It follows Algorithm 3.85, using a combination of normal and gamma distributions
    ///   to generate the sample.
    ///
    /// # Returns
    /// * `f64` - A floating-point value representing the generated random sample.
    ///
    pub fn sample(&mut self) -> f64 {
        match self.degree_of_freedom as usize {
            1_usize => standard_cauchy(&mut self.xyzuv_u_gamma),
            2_usize => {
                // アルゴリズム 3.84 r: = 2
                // step 1
                let mut z = 0_f64;
                let mut w = 0_f64;
                while w == 0_f64 {
                    z = standard_normal(&mut self.xyzuv_n_0, &mut self.xyzuv_n_1);
                    w = standard_exponential(&mut self.xyzuv_u_gamma);
                }
                // step 2
                z / w.sqrt()
            }
            _ => {
                // アルゴリズム 3.85: r > 2
                // step 1
                let z = standard_normal(&mut self.xyzuv_n_0, &mut self.xyzuv_n_1);
                let w = standard_gamma(
                    &mut self.xyzuv_u_gamma,
                    &mut self.xyzuv_n_0_gamma,
                    &mut self.xyzuv_n_1_gamma,
                    &(self.degree_of_freedom as f64 / 2_f64),
                );
                // step 2
                (self.degree_of_freedom as f64).sqrt() * z / w.sqrt()
            }
        }
    }

    /// Updates the degree of freedom parameter for the random variable.
    ///
    /// # Parameters:
    /// * `degree_of_freedom` - The new degree of freedom to set, must be a natural number (r ≥ 1).
    ///
    /// # Returns:
    /// * `Ok(u64)` - If the input `degree_of_freedom` is valid, returns the updated value.
    /// * `Err(&str)` - If the input `degree_of_freedom` is invalid (r < 1), returns an error message.
    ///
    /// # Description:
    /// This method is responsible for setting the degree of freedom (DoF) for the random variable.
    /// The DoF must be a positive integer (natural number) to be valid. If the input value is less
    /// than 1, the function will return an error message and maintain the previous DoF setting.
    /// Otherwise, it updates the degree of freedom with the new valid value and returns it.
    ///
    /// # Example:
    /// ```
    /// let mut dist = rand_simple::TDistribution::new([1192u32, 765u32, 1543u32, 2003u32, 1867u32]);
    /// let result = dist.try_set_params(3_u64);
    /// assert_eq!(result.unwrap(), 3_u64); // Successfully updates the degree of freedom.
    /// ```
    pub fn try_set_params(&mut self, degree_of_freedom: u64) -> Result<u64, &str> {
        // Check if the provided degree_of_freedom is a valid natural number (r ≥ 1).
        if degree_of_freedom < 1_u64 {
            // Return an error if the input is invalid, without changing the current parameter.
            Err("The degree of freedom must be a natural number. The parameter remains unchanged.")
        } else {
            // Update the degree of freedom with the valid input and return the updated value.
            self.degree_of_freedom = degree_of_freedom;
            Ok(degree_of_freedom)
        }
    }
}

impl std::fmt::Display for TDistribution {
    /// Formatter implementation for the TDistribution struct.
    /// This allows the `println!` macro or other formatting macros
    /// to display the t-distribution in a human-readable format.
    ///
    /// # Arguments
    /// * `f` - A mutable reference to the formatter used by the `println!` macro.
    ///
    /// The implementation writes a formatted string representing the current state of the
    /// `TDistribution` instance, specifically displaying the degree of freedom.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use the `write!` macro to format the output string, specifying the degree of freedom.
        // This will output something like: "T(Degree of Freedom parameter) = T(3)"
        write!(
            f,
            "T(Degree of Freedom parameter) = T({})",
            self.degree_of_freedom
        )?;
        // Return `Ok(())` to indicate that the formatting was successful.
        Ok(())
    }
}
