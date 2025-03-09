use crate::standard_distributions::{generate_random_state, standard_gamma};

/// Beta Distribution
/// # Example
/// ```
/// // Create a new Beta distribution with specified random seeds
/// let mut beta = rand_simple::Beta::new([1192u32, 765u32, 1543u32, 2003u32, 1867u32, 1688u32]);
/// // Check the initial state
/// assert_eq!(format!("{beta}"), "Beta(Shape parameter α, Shape parameter β) = Beta(1, 1)");
/// // Generate a random number
/// println!("Returns a random number -> {}", beta.sample());
///
/// // Change the parameters of the probability variable
/// let shape_alpha: f64 = 2f64;
/// let shape_beta: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = beta.try_set_params(shape_alpha, shape_beta);
/// // Check the updated state
/// assert_eq!(format!("{beta}"), "Beta(Shape parameter α, Shape parameter β) = Beta(2, 1.5)");
/// // Generate another random number
/// println!("Returns a random number -> {}", beta.sample());
/// ```
pub struct Beta {
    xyzuv_u_alpha: [u32; 5],   // 状態変数
    xyzuv_n_0_alpha: [u32; 5], // 状態変数
    xyzuv_n_1_alpha: [u32; 5], // 状態変数
    shape_alpha: f64,          // 形状母数 α

    xyzuv_u_beta: [u32; 5],   // 状態変数
    xyzuv_n_0_beta: [u32; 5], // 状態変数
    xyzuv_n_1_beta: [u32; 5], // 状態変数
    shape_beta: f64,          // 形状母数 β
}

impl Beta {
    // Constructor
    /// * `seeds` - Random seeds. Adjusted within the constructor to ensure uniqueness.
    pub fn new(seeds: [u32; 6_usize]) -> Self {
        // Adjust seeds to ensure uniqueness
        let adjusted_seeds = crate::adjust_seeds!(seeds);

        Self {
            // Alpha parameters
            xyzuv_u_alpha: generate_random_state(adjusted_seeds[0]),
            xyzuv_n_0_alpha: generate_random_state(adjusted_seeds[1]),
            xyzuv_n_1_alpha: generate_random_state(adjusted_seeds[2]),
            shape_alpha: 1_f64,

            // Beta parameters
            xyzuv_u_beta: generate_random_state(adjusted_seeds[3]),
            xyzuv_n_0_beta: generate_random_state(adjusted_seeds[4]),
            xyzuv_n_1_beta: generate_random_state(adjusted_seeds[5]),
            shape_beta: 1_f64,
        }
    }

    // Beta Distribution Sampling
    /// Returns a random number following the Beta distribution
    pub fn sample(&mut self) -> f64 {
        let y1 = standard_gamma(
            &mut self.xyzuv_u_alpha,
            &mut self.xyzuv_n_0_alpha,
            &mut self.xyzuv_n_1_alpha,
            &self.shape_alpha,
        );
        let y2 = standard_gamma(
            &mut self.xyzuv_u_beta,
            &mut self.xyzuv_n_0_beta,
            &mut self.xyzuv_n_1_beta,
            &self.shape_beta,
        );
        y1 / (y1 + y2)
    }

    // Beta Distribution Parameter Update
    /// Updates the parameters of the probability variable
    /// * `shape_alpha` - Shape parameter α
    /// * `shape_beta` - Shape parameter β
    pub fn try_set_params(
        &mut self,
        shape_alpha: f64,
        shape_beta: f64,
    ) -> Result<(f64, f64), &str> {
        if shape_alpha <= 0_f64 {
            Err("Shape parameter α is less than or equal to 0. The parameters of the probability variable will remain unchanged from the previous settings.")
        } else if shape_alpha == 1_f64 / 3_f64 {
            Err("Shape parameter α is 1/3. The parameters of the probability variable will remain unchanged from the previous settings.")
        } else if shape_beta <= 0_f64 {
            Err("Shape parameter β is less than or equal to 0. The parameters of the probability variable will remain unchanged from the previous settings.")
        } else if shape_beta == 1_f64 / 3_f64 {
            Err("Shape parameter β is 1/3. The parameters of the probability variable will remain unchanged from the previous settings.")
        } else {
            self.shape_alpha = shape_alpha;
            self.shape_beta = shape_beta;
            Ok((shape_alpha, shape_beta))
        }
    }
}

// Beta Distribution Display Formatter
/// Formatter for displaying Beta distribution using macros like println!
/// * Struct type
/// * Shape parameter α
/// * Shape parameter β
impl core::fmt::Display for Beta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Beta(Shape parameter α, Shape parameter β) = Beta({}, {})",
            self.shape_alpha, self.shape_beta
        )?;
        Ok(())
    }
}
