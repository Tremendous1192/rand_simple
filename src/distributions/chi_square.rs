use crate::create_state;
use crate::standard_distributions::{
    standard_exponential, standard_gamma, xorshift160_0_open_1_open,
};

/// Chi-Square Distribution
///
/// The Chi-Square distribution represents the distribution of the sum of squares of 𝑟 independent standard normal random variables.
///
/// # Examples
///
/// ```
/// // Create a new Chi-Square distribution with specified seeds
/// let mut chi_square = rand_simple::ChiSquare::new([1192_u32, 765_u32, 1543_u32, 2003_u32]);
///
/// // Output the string representation of the distribution
/// assert_eq!(format!("{chi_square}"), "χ^2(Degree of Freedom parameter) = χ^2(1)");
///
/// // Generate a random number following the distribution with initial settings
/// println!("For initial settings, returns a random number following Chi-Square distribution with 1 degree of freedom -> {}", chi_square.sample());
///
/// // Change the parameters of the random variable
/// let degree_of_freedom: u64 = 2_u64;
/// let result: Result<u64, &str> = chi_square.try_set_params(degree_of_freedom);
///
/// // Output the updated string representation of the distribution
/// assert_eq!(format!("{chi_square}"), "χ^2(Degree of Freedom parameter) = χ^2(2)");
///
/// // Generate a random number following the distribution with updated parameters
/// println!("Generates a random number following Chi-Square distribution with {} degrees of freedom -> {}", degree_of_freedom, chi_square.sample());
/// ```
pub struct ChiSquare {
    xyzuv_u_gamma: [u32; 5],   // State variables
    xyzuv_n_0_gamma: [u32; 5], // State variables
    xyzuv_n_1_gamma: [u32; 5], // State variables

    xyzuv_uniform: [u32; 5], // State variables

    degree_of_freedom: u64, // Degrees of freedom (𝑟)
    r_div2: f64,            // Preprocessing
}

impl ChiSquare {
    /// Chi-Square Distribution Constructor
    ///
    /// This constructor initializes a new instance of the Chi-Square distribution with the given seeds for random number generation.
    ///
    /// * `_seed` - Seeds for random number generation
    ///
    /// # Examples
    ///
    /// ```
    /// // Create a new Chi-Square distribution with specified seeds
    /// let mut chi_square = rand_simple::ChiSquare::new([1192_u32, 765_u32, 1543_u32, 2003_u32]);
    /// ```
    ///
    pub fn new(seeds: [u32; 4_usize]) -> Self {
        // Adjusting seeds to ensure uniqueness
        let adjusted_seeds = crate::adjust_seeds!(seeds);

        Self {
            xyzuv_u_gamma: create_state(adjusted_seeds[0]), // State variable for gamma distribution 1
            xyzuv_n_0_gamma: create_state(adjusted_seeds[1]), // State variable for gamma distribution 2
            xyzuv_n_1_gamma: create_state(adjusted_seeds[2]), // State variable for gamma distribution 3

            xyzuv_uniform: create_state(adjusted_seeds[3]), // State variable for uniform distribution

            degree_of_freedom: 1u64, // Degrees of freedom
            r_div2: 0.5f64,          // Preprocessing for random number calculation
        }
    }

    /// Generates a Random Number
    ///
    /// This method generates a random number based on the Chi-Square distribution algorithm.
    ///
    /// Algorithm 3.79:
    /// - When the degree of freedom is 2, the distribution becomes an Exponential distribution Exp(2).
    /// - For r > 1, it generates a random number X following the Gamma distribution Γ(r_div2, 2).
    /// - For r = 1, it generates a random number Y following the Gamma distribution Γ(3/2, 2), then generates a uniform random number U in the interval (0, 1), and finally calculates X = 2YU^2.
    ///
    /// # Returns
    ///
    /// The generated random number.
    ///
    pub fn sample(&mut self) -> f64 {
        if self.degree_of_freedom == 2u64 {
            // When degree of freedom is 2, it is equivalent to Exponential distribution Exp(2)
            2f64 * standard_exponential(&mut self.xyzuv_uniform)
        } else if self.degree_of_freedom > 1u64 {
            // For r > 1, generates random number X following Gamma distribution Γ(r_div2, 2)
            standard_gamma(
                &mut self.xyzuv_u_gamma,
                &mut self.xyzuv_n_0_gamma,
                &mut self.xyzuv_n_1_gamma,
                &self.r_div2,
            ) * 2f64
        } else {
            // For r = 1, generates random number Y following Gamma distribution Γ(3/2, 2)
            // Generates a uniform random number U in the interval (0, 1)
            // Calculates X = 2YU^2
            let y = standard_gamma(
                &mut self.xyzuv_u_gamma,
                &mut self.xyzuv_n_0_gamma,
                &mut self.xyzuv_n_1_gamma,
                &(3f64 / 2f64),
            ) * 2f64;
            let u = xorshift160_0_open_1_open(&mut self.xyzuv_uniform);
            u.powi(2) * y * 2_f64
        }
    }

    /// Sets the Parameters of the Probability Variable
    ///
    /// This method allows for changing the parameters of the Chi-Square distribution.
    ///
    /// # Arguments
    ///
    /// * `degree_of_freedom` - The degree of freedom (r) parameter of the distribution.
    ///
    /// # Returns
    ///
    /// Returns `Ok(degree_of_freedom)` if successful, else returns an error message.
    ///
    pub fn try_set_params(&mut self, degree_of_freedom: u64) -> Result<u64, &str> {
        if degree_of_freedom < 1_u64 {
            // Degree of freedom must be a natural number
            Err("The degree of freedom must be a natural number. The parameters of the probability variable will remain unchanged.")
        } else {
            // Sets the degree of freedom and calculates r_div2
            self.degree_of_freedom = degree_of_freedom;
            self.r_div2 = degree_of_freedom as f64 / 2f64;
            Ok(degree_of_freedom)
        }
    }
}

/// Formatter for Displaying the Chi-Square Distribution
///
/// This implementation enables the display of Chi-Square distributions in macros like `print!`.
///
/// # Arguments
///
/// * `f` - The formatter to write the formatted string to.
///
/// # Returns
///
/// Returns `core::fmt::Result`.
///
impl core::fmt::Display for ChiSquare {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Writes the formatted string to the formatter
        write!(
            f,
            "χ^2(Degree of Freedom parameter) = χ^2({})",
            self.degree_of_freedom
        )?;
        Ok(())
    }
}
