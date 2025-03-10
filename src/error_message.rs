use std::fmt;
//use std::io;// 標準エラーのfromを実装する場合に使用する。

/// Enum representing possible errors when updating a parameter.
#[derive(Debug)]
pub enum ParameterUpdateError {
    /// Error for invalid range configuration within a closed interval `[min, max]`.
    /// This occurs when the minimum value is greater than the maximum value.
    ParameterRangeErrorF64 { min: f64, max: f64 },
}

impl fmt::Display for ParameterUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParameterRangeErrorF64 { min, max } => write!(
                f,
                "Minimum value {min:?} must be less than maximum value {max:?}."
            ),
        }
    }
}

/// Unit test to verify the behavior of `ParameterUpdateError`.
#[test]
fn test_parameter_update_error() {
    // Create an instance of the error with an incorrect range configuration.
    let error = ParameterUpdateError::ParameterRangeErrorF64 {
        min: 11_f64, // Invalid: minimum is greater than maximum
        max: -1_f64,
    };

    // Verify that the error message matches the expected string.
    assert_eq!(
        error.to_string(),
        "Minimum value 11.0 must be less than maximum value -1.0." // Expected output
    );
}
