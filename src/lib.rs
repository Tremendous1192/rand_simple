#![doc = include_str!("../README.md")]

mod distributions; // 確率変数の詳細
                   //#[cfg(test)]
                   //mod sandbox; // 試行錯誤するためのテストモジュール
mod standard_distributions; // 標準分布を計算するモジュール
                            //#[cfg(test)]
                            //mod test_distributions; // 機能確認のためのテストモジュール
mod error_message; // パラメータ更新時のエラーメッセージモジュール

// エラーメッセージ
pub use crate::error_message::ParameterUpdateError;

// 共通処理

#[macro_export]
/// Macro to generate an array of random seeds in a `std` environment.
///
/// This macro produces a `u32` array of a specified length, where each element is derived from
/// the current system time since the UNIX epoch. The generated seeds can be used for initializing
/// random number generators.
///
/// # Parameters
/// - `$length: usize` - The length of the array to be generated.
///
/// # Implementation Details
/// 1. **Initialization**:
///    - A mutable array `array` of size `$length` is created and initialized with zeros (`[0_u32; $length]`).
///
/// 2. **System Time**:
///    - The current system time is retrieved using `std::time::SystemTime::now()`.
///    - The duration since the UNIX epoch is calculated using `duration_since(std::time::UNIX_EPOCH)`.
///    - If the system clock is misconfigured (e.g., time moves backwards), it will panic with an error message.
///
/// 3. **Seed Generation Logic**:
///    - Each element of the array is assigned a value based on the index `i` modulo 6. This ensures
///      that a mix of time-based values is used to generate diverse seeds:
///        - **Case 0**: The seed is derived from the number of milliseconds since the epoch (`as_millis`).
///        - **Case 1**: The seed is the maximum `u32` value minus the nanoseconds since the epoch (`as_nanos`).
///        - **Case 2**: The seed is the number of seconds since the epoch divided by 60.
///        - **Case 3**: The seed is the maximum `u32` value minus the microseconds since the epoch (`as_micros`).
///        - **Case 4**: The seed is the number of seconds since the epoch.
///        - **Case 5**: The seed is the number of milliseconds since the epoch divided by 60.
///        - **Default Case**: If the index doesn't match any case (which is improbable), it assigns a fixed value of `1_192_765_u32`.
///
/// 4. **Return Value**:
///    - The populated array is returned, providing a set of random seeds.
///
/// # Example Usage
/// ```
/// use rand_simple::generate_seeds;
/// // Generate an array of 5 random seeds
/// let seeds = generate_seeds!(5);
/// println!("Generated seeds: {:?}", seeds);
///
/// // The resulting `seeds` array will contain 5 `u32` values derived from the current system time.
/// ```
///
/// # Notes
/// - The seeds generated are dependent on the system time and provide a level of randomness sufficient for most applications.
/// - This macro is designed to work in environments where `std` is available, as it relies on `std::time::SystemTime`.
macro_rules! generate_seeds {
    ($length: expr) => {{
        // 状態変数を保存する配列を保存する。
        let mut array = [0_u32; $length];
        // UNIX_EPOCH からの経過時間を基に乱数の種を計算する。
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        for i in 0..array.len() {
            array[i] = match i % 6_usize {
                0_usize => (duration.as_millis() & 0xFFFF_FFFF) as u32,
                1_usize => std::u32::MAX - (duration.as_nanos() & 0xFFFF_FFFF) as u32,
                2_usize => ((duration.as_secs() & 0xFFFF_FFFF) as u32) / 60_u32,
                3_usize => std::u32::MAX - (duration.as_micros() & 0xFFFF_FFFF) as u32,
                4_usize => (duration.as_secs() & 0xFFFF_FFFF) as u32,
                5_usize => ((duration.as_millis() & 0xFFFF_FFFF) as u32) / 60_u32,
                _ => 1_192_765_u32,
            };
        }
        array
    }};
}

/// 現在時刻に基づいて乱数の種を作成するマクロの動作確認
#[test]
fn test_generate_seeds() {
    let seeds6: [u32; 6] = generate_seeds!(6);
    println!("{:#?}", seeds6);
}

/// 独立な乱数を生成するために、配列の要素を互いに異なる値に変更するマクロ
/// # 引数
/// 乱数の種の配列
/// # 戻り値
/// 互いに異なる値に修正した乱数の種の配列
macro_rules! adjust_seeds {
    ($array:expr) => {{
        // 値を上書きするために配列のコピーを作成する。
        let mut copy_array = $array;
        // 互いに異なる値であるかを確認するために2重ループを使用する。
        for i in 0..(copy_array.len() - 1) {
            for j in (i + 1)..copy_array.len() {
                // 乱数の種が等しい場合ビット演算で値を変更する
                if copy_array[i] == copy_array[j] {
                    copy_array[j] = (copy_array[j] << 3) ^ (copy_array[i] >> 2);
                    // ビット演算後の値が0になってしまった場合、マジックナンバーを代入する
                    if copy_array[j] == 0 {
                        copy_array[j] = 1192;
                    }
                }
            }
        }
        copy_array
    }};
}
pub(crate) use adjust_seeds;

// 連続型確率変数

// 一様乱数
pub use crate::distributions::uniform::Uniform;

// 正規分布
pub use crate::distributions::normal::Normal;

// 半正規分布
pub use crate::distributions::half_normal::HalfNormal;

// 対数正規分布
pub use crate::distributions::log_normal::LogNormal;

// コーシー分布
pub use crate::distributions::cauchy::Cauchy;

// 半コーシー分布
pub use crate::distributions::half_cauchy::HalfCauchy;

// レヴィ分布
pub use crate::distributions::levy::Levy;

// 指数分布
pub use crate::distributions::exponential::Exponential;

// ラプラス分布
pub use crate::distributions::laplace::Laplace;

// 対数ラプラス分布
pub use crate::distributions::log_laplace::LogLaplace;

// レイリー分布
pub use crate::distributions::rayleigh::Rayleigh;

// ワイブル分布
pub use crate::distributions::weibull::Weibull;

// 反射ワイブル分布
pub use crate::distributions::reflected_weibull::ReflectedWeibull;

// フレシェ分布
pub use crate::distributions::frechet::Frechet;

// ガンベル分布
pub use crate::distributions::gunbel::Gunbel;

// ガンマ分布
pub use crate::distributions::gamma::Gamma;

// ベータ分布
pub use crate::distributions::beta::Beta;

// ディリクレ分布
//pub struct Dirichlet {}

// べき関数分布
pub use crate::distributions::power_function::PowerFunction;

// 指数べき分布
//pub struct ExponentialPower {}

// アーラン分布
pub use crate::distributions::erlang::Erlang;

// χ二乗分布
pub use crate::distributions::chi_square::ChiSquare;

// χ分布
pub use crate::distributions::chi::Chi;

// F分布
pub use crate::distributions::f::FDistribution;

// t分布
pub use crate::distributions::t::TDistribution;

// 逆ガウス分布
pub use crate::distributions::inverse_gaussian::InverseGaussian;

// 三角分布
pub use crate::distributions::triangular::Triangular;

// パレート分布
//pub struct Pareto {}

// ロジスティック分布
//pub struct Logistic {}

// 双曲線正割分布
//pub struct HeyperbolicSecant {}

// 余弦分布
//pub struct RaisedCosine {}

// 逆正弦分布
//pub struct Arcsine {}

// フォン・ミーゼス分布
//pub struct VonMises {}

// 非心ガンマ分布
//pub struct NonCentralGamma {}

// 非心ベータ分布
//pub struct NonCentralBeta {}

// 非心ガンマ二乗分布
//pub struct NonCentralChiSquare {}

// 非心ガンマ分布
//pub struct NonCentralChi {}

// 非心F分布
//pub struct NonCentralF {}

// 非心t分布
//pub struct NonCentralT {}

// プランク分布
//pub struct Plank {}

// 離散型確率変数

// ベルヌーイ分布
pub use crate::distributions::bernoulli::Bernoulli;

// 二項分布
//pub  struct Binomial {}

// 幾何分布
pub use crate::distributions::geometric::Geometric;

// ポアソン分布
//pub struct Poisson {}

// 超幾何分布
//pub struct HeyperGeometric {}

// 多項分布
//pub struct Multinominal {}

// 負の二項分布
//pub struct NegativeBinomial {}

// 負の超幾何分布
//pub struct NegativeHeyperGeometric {}

// 対数級数分布
//pub struct LogarithmicSeries {}

// ユール・シモン分布
//pub struct YuleSimon {}

// ジップ・マンデルブロート分布
//pub struct ZipfMandelbrot {}

// ゼータ分布
//pub struct Zeta {}
