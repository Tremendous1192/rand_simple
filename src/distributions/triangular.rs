use crate::create_state;
use crate::standard_distributions::xorshift160_0_1;

/// Represents a triangular distribution defined by a minimum value, maximum value, and mode.
///
/// # Example Usage
/// ```
/// let mut triangular = rand_simple::Triangular::new(1192_u32);
///
/// // Default configuration
/// assert_eq!(format!("{triangular}"), "TRI(Min, Max, Mode) = TRI(0, 1, 0.5)");
/// println!(
///     "Generates a random number following a triangular distribution over [0, 1] with mode 0.5 -> {}",
///     triangular.sample()
/// );
///
/// // Modify distribution parameters
/// let min: f64 = -1_f64;
/// let max: f64 = 1_f64;
/// let mode: f64 = 0.25_f64;
/// let result: Result<(f64, f64, f64), &str> = triangular.try_set_params(min, max, mode);
///
/// assert_eq!(format!("{triangular}"), "TRI(Min, Max, Mode) = TRI(-1, 1, 0.25)");
/// println!(
///     "Generates a random number following a triangular distribution over [{}, {}] with mode {} -> {}",
///     min, max, mode, triangular.sample()
/// );
/// ```
///
/// # Fields
/// - `xyzuv`: A state variable used for random number generation.
/// - `min`: The minimum value of the distribution, defining the left endpoint of the interval.
/// - `max`: The maximum value of the distribution, defining the right endpoint of the interval.
/// - `mode`: The mode (peak) of the distribution, indicating the value where the distribution reaches its maximum density.
pub struct Triangular {
    xyzuv: [u32; 5], // 状態変数
    min: f64,        // 最小値
    max: f64,        // 最大値
    mode: f64,       // モード
}

impl Triangular {
    /// Constructor for the `Triangular` struct.
    /// Initializes the random number generator state and sets default parameters.
    ///
    /// # Arguments
    /// * `_seed` - A seed value used to initialize the random number generator state.
    ///
    /// # Default Parameters
    /// - `min`: 0.0 (lower bound of the interval)
    /// - `max`: 1.0 (upper bound of the interval)
    /// - `mode`: 0.5 (most likely value within the interval)
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzuv: create_state(_seed),
            min: 0_f64,
            max: 1_f64,
            mode: 0.5_f64,
        }
    }

    /// Generates a random number following the triangular distribution.
    ///
    /// This method uses Algorithm 3.95 (Inverse Transform Sampling) to generate a random value
    /// based on the current `min`, `max`, and `mode` parameters.
    /// # Returns
    /// A random number following the triangular distribution.
    pub fn sample(&mut self) -> f64 {
        // アルゴリズム 3.95 (逆関数法)
        let s = self.max - self.min;
        let d = (self.mode - self.min) / s;
        let d_1m = 1_f64 - d;

        // step 1
        let u = xorshift160_0_1(&mut self.xyzuv);

        // step 2
        let y = if u < d {
            (d * u).sqrt()
        } else {
            1_f64 - (d_1m * (1_f64 - u)).sqrt()
        };

        // step 3
        self.min + s * y
    }

    /// Updates the parameters of the triangular distribution.
    ///
    /// # Arguments
    /// * `min` - The minimum value of the distribution.
    /// * `max` - The maximum value of the distribution.
    /// * `mode` - The mode (most probable value) of the distribution.
    ///
    /// # Validation
    /// - The `min` must be less than the `max`.
    /// - The `mode` must lie within the interval `[min, max]`.
    ///
    /// # Returns
    /// - `Ok((min, max, mode))` if the parameters are valid.
    /// - `Err(&str)` if the parameters are invalid, with an error message explaining the issue.
    pub fn try_set_params(
        &mut self,
        min: f64,
        max: f64,
        mode: f64,
    ) -> Result<(f64, f64, f64), &str> {
        if min >= max {
            Err("最小値と最大値が等しい、あるいは最小値の方が大きいです。確率変数のパラメータは前回の設定を維持します。")
        } else if mode < min || max < mode {
            Err("モードが最小値よりも小さい、あるいはモードが最大値よりも大きいです。確率変数のパラメータは前回の設定を維持します。")
        } else {
            self.min = min;
            self.max = max;
            self.mode = mode;
            Ok((self.min, self.max, self.mode))
        }
    }
}

impl core::fmt::Display for Triangular {
    /// Formatter for displaying with println! macro and others.
    /// * Range (Closed Interval)
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "TRI(Min, Max, Mode) = TRI({}, {}, {})",
            self.min, self.max, self.mode
        )?;
        Ok(())
    }
}
