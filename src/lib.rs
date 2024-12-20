#![doc = include_str!("../README.md")]

mod distributions; // 確率変数の詳細
mod standard_distributions; // 標準分布を計算するモジュール
                            //#[cfg(test)] mod test_distributions; // 機能確認のためのテストモジュール
#[cfg(test)]
mod sandbox; // 試行錯誤するためのテストモジュール

// 状態変数(x, y, z, u, v)を設定する
// 下記の論文の初期値を参考にする
// https://www.researchgate.net/publication/5142825_Xorshift_RNGs
pub(crate) fn create_state(_seed: u32) -> [u32; 5] {
    [123456789, 362436069, 521288629, 88675123, _seed]
}

// 共通処理

#[macro_export]
/// std環境で乱数の種の配列を生成するマクロ
/// * `$length: usize` - 配列の長さ
macro_rules! generate_seeds {
    ($length: expr) => {{
        let mut array = [0_u32; $length];
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        for i in 0..array.len() {
            array[i] = match i % 6_usize {
                0_usize => duration.as_millis() as u32,
                1_usize => std::u32::MAX - duration.as_nanos() as u32,
                2_usize => (duration.as_secs() as u32) / 60_u32,
                3_usize => std::u32::MAX - duration.as_micros() as u32,
                4_usize => duration.as_secs() as u32,
                5_usize => (duration.as_millis() as u32) / 60_u32,
                _ => 1_192_765_u32,
            };
        }
        array
    }};
}

// 配列の要素を互いに異なる値に変更するマクロ
macro_rules! adjust_seeds {
    ($array:expr) => {{
        let mut copy_array = $array;
        for i in 0..(copy_array.len() - 1) {
            for j in (i + 1)..copy_array.len() {
                if copy_array[i] == copy_array[j] {
                    copy_array[j] = (copy_array[j] << 3) ^ (copy_array[i] >> 2);
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

/// 三角分布
/// # 使用例
/// ```
/// let mut triangular = rand_simple::Triangular::new(1192_u32);
/// println!("閉区間[0, 1], モード 0.5の三角分布に従う乱数を返す -> {}", triangular.sample());
///
/// // 確率変数のパラメータを変更する
/// let min: f64 = -1_f64;
/// let max: f64 = 1_f64;
/// let mode: f64 = 0.25_f64;
/// let result: Result<(f64, f64, f64), &str> = triangular.try_set_params(min, max, mode);
/// println!("閉区間[{}, {}], モード {}の三角分布に従う乱数を返す -> {}", min, max, mode, triangular.sample());
/// ```
pub struct Triangular {
    xyzuv: [u32; 5], // 状態変数
    min: f64,        // 最小値
    max: f64,        // 最大値
    mode: f64,       // モード
}

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

/// ベルヌーイ分布
/// # 使用例
/// ```
/// let mut bernoulli = rand_simple::Bernoulli::new(1192u32);
/// println!("発生確率 θ = 0.5 の事象が生じたか(1)、否か(0)の判定 -> {}", bernoulli.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let probability: f64 = 0.8f64;
/// let result: Result<f64, &str> = bernoulli.try_set_params(probability);
/// println!("発生確率 θ = {} の事象が生じたか(1)、否か(0)の判定 -> {}", probability, bernoulli.sample());
/// ```
pub struct Bernoulli {
    xyzuv: [u32; 5],  // 状態変数
    probability: f64, // 発生確率
}

// 二項分布
//pub  struct Binomial {}

/// 幾何分布
/// # 使用例
/// ```
/// let mut geometric = rand_simple::Geometric::new(1192u32);
/// println!("発生確率 θ = 0.5 の事象が生じるまでの試行回数 -> {}", geometric.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let probability: f64 = 0.8f64;
/// let result: Result<f64, &str> = geometric.try_set_params(probability);
/// println!("発生確率 θ = {} の事象が生じるまでの試行回数 -> {}", probability, geometric.sample());
/// ```
pub struct Geometric {
    xyzuv: [u32; 5],  // 状態変数
    probability: f64, // 発生確率
}

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
