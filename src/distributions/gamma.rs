use crate::create_state;
use crate::standard_distributions::standard_gamma;

// Gamma Distribution
/// Represents a gamma distribution.
/// # Example
/// ```
/// let mut gamma = rand_simple::Gamma::new([1192u32, 765u32, 1543u32]);
/// assert_eq!(format!("{gamma}"), "Γ(Shape parameter, Scale parameter) = Γ(1, 1)");
/// println!("Generating a random number following the standard gamma distribution with shape parameter α = 1 and scale parameter β = 1 -> {}", gamma.sample());
///
/// // Changing the parameters of the random variable
/// let shape: f64 = 2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = gamma.try_set_params(shape, scale);
/// assert_eq!(format!("{gamma}"), "Γ(Shape parameter, Scale parameter) = Γ(2, 1.5)");
/// println!("Generating a random number following the gamma distribution with shape parameter α = {}, and scale parameter β = {} -> {}", shape, scale, gamma.sample());
/// ```
pub struct Gamma {
    xyzuv_u: [u32; 5],   // State variables
    xyzuv_n_0: [u32; 5], // State variables
    xyzuv_n_1: [u32; 5], // State variables
    shape: f64,          // Shape parameter α
    scale: f64,          // Scale parameter β
}

impl Gamma {
    // Constructor
    /// Constructs a new instance of the random number generator.
    /// * `seeds` - Seeds for the random number generator. Adjusted internally to ensure uniqueness.
    pub fn new(seeds: [u32; 3]) -> Self {
        // Adjust the seeds to ensure uniqueness
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        // Create a new instance of the random number generator
        Self {
            xyzuv_u: create_state(adjusted_seeds[0]),
            xyzuv_n_0: create_state(adjusted_seeds[1]),
            xyzuv_n_1: create_state(adjusted_seeds[2]),
            shape: 1_f64,
            scale: 1_f64,
        }
    }

    // Function to compute random numbers
    /// It computes a random number using the standard gamma distribution.
    pub fn sample(&mut self) -> f64 {
        // Generate a random number using the standard gamma distribution
        let random_number = standard_gamma(
            &mut self.xyzuv_u,
            &mut self.xyzuv_n_0,
            &mut self.xyzuv_n_1,
            &self.shape,
        );
        // Scale the random number by the scale parameter and return it
        random_number * self.scale
    }

    // Function to change the parameters of the probability variable
    /// * `shape` - Shape parameter
    /// * `scale` - Scale parameter
    pub fn try_set_params(&mut self, shape: f64, scale: f64) -> Result<(f64, f64), &str> {
        // Check if the shape parameter is less than or equal to 0
        if shape <= 0_f64 {
            // Return an error if the shape parameter is less than or equal to 0
            Err("The shape parameter is less than or equal to 0. The parameters of the probability variable will remain unchanged.")
        }
        // Check if the shape parameter is equal to 1/3
        else if shape == 1_f64 / 3_f64 {
            // Return an error if the shape parameter is equal to 1/3
            Err("The shape parameter is 1/3. The parameters of the probability variable will remain unchanged.")
        }
        // Check if the scale parameter is less than or equal to 0
        else if scale <= 0_f64 {
            // Return an error if the scale parameter is less than or equal to 0
            Err("The scale parameter is less than or equal to 0. The parameters of the probability variable will remain unchanged.")
        } else {
            // Set the new shape and scale parameters
            self.shape = shape;
            self.scale = scale;
            // Return a successful result with the new parameters
            Ok((shape, scale))
        }
    }
}

impl core::fmt::Display for Gamma {
    /// Formatter for displaying in macros like print!
    /// * Shape parameter
    /// * Scale parameter
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Γ(Shape parameter, Scale parameter) = Γ({}, {})",
            self.shape, self.scale
        )?;
        Ok(())
    }
}
