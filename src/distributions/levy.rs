use crate::standard_distributions::{generate_random_state, standard_normal};

/// Lévy Distribution
/// # Example
/// ```
/// // Create a new Lévy distribution with a location parameter (μ) of 0 and a scale parameter (θ) of 1
/// let mut levy = rand_simple::Levy::new([1192_u32, 765_u32]);
/// assert_eq!(format!("{levy}"), "Lévy(Location parameter, Scale parameter) = Lévy(0, 1)");
/// println!("Returns a random number following a standard Levy distribution -> {}", levy.sample());
///
/// // Modify the distribution's parameters
/// let location: f64 = -2_f64;
/// let scale: f64 = 1.5_f64;
/// // Update the parameters and generate a random number following the modified Levy distribution
/// let result: Result<(f64, f64), &str> = levy.try_set_params(location, scale);
/// assert_eq!(format!("{levy}"), "Lévy(Location parameter, Scale parameter) = Lévy(-2, 1.5)");
/// println!("Returns a random number following a Levy distribution with location μ = {} and scale θ = {} -> {}", location, scale, levy.sample());
/// ```
pub struct Levy {
    xyzuv0: [u32; 5], // 状態変数
    xyzuv1: [u32; 5], // 状態変数
    location: f64,    // 位置母数
    scale: f64,       // 尺度母数
}

impl Levy {
    /// Constructor
    /// * `seeds` - Random number seeds. Adjusted internally to ensure uniqueness.
    pub fn new(seeds: [u32; 2_usize]) -> Self {
        let adjusted_seeds = crate::adjust_seeds!(seeds);
        Self {
            xyzuv0: generate_random_state(adjusted_seeds[0]),
            xyzuv1: generate_random_state(adjusted_seeds[1]),
            location: 0_f64,
            scale: 1_f64,
        }
    }

    /// Generate a random number.
    pub fn sample(&mut self) -> f64 {
        loop {
            let z = standard_normal(&mut self.xyzuv0, &mut self.xyzuv1).abs();
            if z > 0_f64 {
                return z.powi(-2_i32) * self.scale + self.location;
            }
        }
    }

    /// Modify the parameters of the probability variable.
    /// * `location` - Location parameter
    /// * `scale` - Scale parameter
    pub fn try_set_params(
        &mut self,
        location: f64,
        scale: f64,
    ) -> Result<(f64, f64), &'static str> {
        if scale <= 0_f64 {
            Err("The scale parameter is less than or equal to 0. The parameters of the probability variable remain unchanged.")
        } else {
            self.location = location;
            self.scale = scale;
            Ok((location, scale))
        }
    }
}

impl core::fmt::Display for Levy {
    /// Formatter for use with macros like `println!`
    /// * Location parameter
    /// * Scale parameter
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Lévy(Location parameter, Scale parameter) = Lévy({}, {})",
            self.location, self.scale
        )?;
        Ok(())
    }
}
