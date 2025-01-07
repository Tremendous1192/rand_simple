/// エラーメッセージ
pub enum ParameterUpdateError {
    /// 閉区間 ```[min, max]``` の設定ミス
    ParameterRangeErrorF64 { min: f64, max: f64 },
    //InvalidParameter(String),
    //MissingParameter(String),
    //ImmutableParameter(String),
    //ParameterRangeError { parameter: String, min: i32, max: i32 },
}

impl core::fmt::Display for ParameterUpdateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParameterUpdateError::ParameterRangeErrorF64 { min, max } => {
                write!(
                    f,
                    "最小値 {} は最大値 {} よりも小さくしてください",
                    min, max
                )
            } /*
              ParameterUpdateError::InvalidParameter(param) => {
                  write!(f, "Invalid parameter: '{}'", param)
              }
              ParameterUpdateError::MissingParameter(param) => {
                  write!(f, "Missing required parameter: '{}'", param)
              }
              ParameterUpdateError::ImmutableParameter(param) => {
                  write!(f, "Parameter '{}' is immutable and cannot be updated", param)
              }
              ParameterUpdateError::ParameterRangeError { parameter, min, max } => {
                  write!(
                      f,
                      "Parameter '{}' is out of range. Valid range is {} to {}",
                      parameter, min, max
                  )
              }*/
        }
    }
}

/// エラーメッセージの動作確認
#[test]
fn test_parameter_update_error() {
    let error = ParameterUpdateError::ParameterRangeErrorF64 {
        min: 11_f64,
        max: -1_f64,
    };
    println!("{}", error);
}
