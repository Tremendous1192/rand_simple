use crate::create_state;
use crate::standard_distributions::{standard_gamma, xorshift160_0_1_open};

/// Chi Distribution
///
/// This struct represents the Chi distribution, which is a special case of the gamma distribution
/// with a shape parameter equal to half the degrees of freedom. It is useful in various statistical
/// applications, including hypothesis testing and confidence interval estimation.
///
/// # Example
///
/// ```
/// let mut chi = rand_simple::Chi::new([1192_u32, 765_u32, 1543_u32, 2003_u32]);
///
/// // Verify the initial state
/// assert_eq!(format!("{chi}"), "χ(Degree of Freedom parameter) = χ(1)");
///
/// // Generate a random number from the Chi distribution with 1 degree of freedom
/// println!("Initial setting: Random number from Chi distribution with 1 degree of freedom -> {}", chi.sample());
///
/// // Change the degrees of freedom
/// let degree_of_freedom: u64 = 2_u64;
/// let result: Result<u64, &str> = chi.try_set_params(degree_of_freedom);
///
/// // Verify the new state
/// assert_eq!(format!("{chi}"), "χ(Degree of Freedom parameter) = χ(2)");
///
/// // Generate a random number from the Chi distribution with the updated degrees of freedom
/// println!("Updated setting: Random number from Chi distribution with {} degrees of freedom -> {}", degree_of_freedom, chi.sample());
/// ```
///
/// # Fields
///
/// * `xyzuv_u_gamma` - State variable for gamma distribution random number generation
/// * `xyzuv_n_0_gamma` - State variable for gamma distribution random number generation
/// * `xyzuv_n_1_gamma` - State variable for gamma distribution random number generation
/// * `xyzuv_uniform` - State variable for uniform distribution random number generation
/// * `degree_of_freedom` - Degrees of freedom, must be a positive integer
///
pub struct Chi {
    xyzuv_u_gamma: [u32; 5], // State variable for gamma distribution random number generation
    xyzuv_n_0_gamma: [u32; 5], // State variable for gamma distribution random number generation
    xyzuv_n_1_gamma: [u32; 5], // State variable for gamma distribution random number generation

    xyzuv_uniform: [u32; 5], // State variable for uniform distribution random number generation

    degree_of_freedom: u64, // Degrees of freedom, must be a positive integer
}

impl Chi {
    /// Constructor for the `Chi` struct
    /// # Arguments
    ///
    /// * `seeds` - An array of 4 unsigned 32-bit integers used as seeds for the random number generators.
    ///             The seeds are adjusted to ensure they are unique and suitable for the random number generation.
    pub fn new(seeds: [u32; 4_usize]) -> Self {
        // Adjust the seeds to ensure they are suitable for the random number generation
        let adjusted_seeds = crate::adjust_seeds!(seeds);

        // Create a new instance of the Chi struct with the adjusted seeds
        Self {
            // Initialize the state variables for gamma distribution random number generation
            xyzuv_u_gamma: create_state(adjusted_seeds[0]), // Seed for gamma distribution random number generator 1
            xyzuv_n_0_gamma: create_state(adjusted_seeds[1]), // Seed for gamma distribution random number generator 2
            xyzuv_n_1_gamma: create_state(adjusted_seeds[2]), // Seed for gamma distribution random number generator 3

            // Initialize the state variable for uniform distribution random number generation
            xyzuv_uniform: create_state(adjusted_seeds[3]), // Seed for uniform distribution random number generator

            // Set the initial degrees of freedom
            degree_of_freedom: 1_u64, // Default degrees of freedom
        }
    }

    /// Generates a random number following the Chi distribution.
    pub fn sample(&mut self) -> f64 {
        // Step 1: Generate a random number Y following the Chi-squared distribution (χ^2(x))
        let y = if self.degree_of_freedom > 1_u64 {
            // If degrees of freedom > 1, generate a gamma distribution random number with the degrees of freedom as the shape parameter
            standard_gamma(
                &mut self.xyzuv_u_gamma,
                &mut self.xyzuv_n_0_gamma,
                &mut self.xyzuv_n_1_gamma,
                &(self.degree_of_freedom as f64),
            ) * 2_f64
        } else {
            // If degrees of freedom == 1, generate a random number using a specific method
            let y = standard_gamma(
                &mut self.xyzuv_u_gamma,
                &mut self.xyzuv_n_0_gamma,
                &mut self.xyzuv_n_1_gamma,
                &(3_f64 / 2_f64),
            ) * 2_f64;
            let u = xorshift160_0_1_open(&mut self.xyzuv_uniform); // Generate a uniform random number in the interval (0, 1)
            u.powi(2) * y * 2_f64
        };
        // Step 2: Calculate X = √Y
        y.sqrt() // Return the square root of Y as the final random number
    }

    /// Updates the parameters of the random variable.
    /// # Parameters
    ///
    /// * `degree_of_freedom` - The degrees of freedom `r` (should be a natural number).
    pub fn try_set_params(&mut self, degree_of_freedom: u64) -> Result<u64, &str> {
        // Step 1: Validate the degrees of freedom
        if degree_of_freedom < 1_u64 {
            // If the degrees of freedom is less than 1, return an error message
            Err("自由度は自然数である必要があります。確率変数のパラメータは前回の設定を維持します。")
        } else {
            // Step 2: Update the internal state with the new degrees of freedom
            self.degree_of_freedom = degree_of_freedom;
            // Return the updated degrees of freedom
            Ok(degree_of_freedom)
        }
    }
}

impl core::fmt::Display for Chi {
    /// Formatter for displaying in macros like println!
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Writes the formatted string to the formatter
        write!(
            f,
            "χ(Degree of Freedom parameter) = χ({})",
            self.degree_of_freedom
        )?;
        Ok(())
    }
}
