#![doc = include_str!("../README.md")]

mod distributions; // 確率変数の詳細
mod standard_distributions; // 標準分布を計算するモジュール
                            //#[cfg(test)] mod test_distributions; // 機能確認のためのテストモジュール
#[cfg(test)]
mod sandbox; // 試行錯誤するためのテストモジュール
use std::time::{SystemTime, UNIX_EPOCH}; // 時刻の取得

// 状態変数(x, y, z, u, v)を設定する
// 下記の論文の初期値を参考にする
// https://www.researchgate.net/publication/5142825_Xorshift_RNGs
pub(crate) fn create_state(_seed: u32) -> [u32; 5] {
    [123456789, 362436069, 521288629, 88675123, _seed]
}

// 共通処理
/// 現在時刻から乱数の種を計算する関数
pub fn create_seed() -> u32 {
    // 4_294_967_295u32 / 24 * 60 * 60 * 1000ミリ秒/日 ≒ 49.7日周期
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u32
}

// 共通処理
/// 正規分布等2つの乱数の種が必要な確率変数に対して、現在時刻から乱数の種を計算する
pub fn create_seeds_duo() -> [u32; 2] {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    // 49.7日周期と4秒周期の組み合わせ
    [
        duration.as_millis() as u32,
        std::u32::MAX - duration.as_nanos() as u32,
    ]
}

/// ガンマ分布等3つの乱数の種が必要な確率変数に対して、現在時刻から乱数の種を計算する
pub fn create_seeds_trio() -> [u32; 3] {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    // 49.7日周期と136年周期と4秒周期の組み合わせ
    [
        duration.as_millis() as u32,
        duration.as_secs() as u32,
        std::u32::MAX - duration.as_nanos() as u32,
    ]
}

/// ベータ分布等6つの乱数の種が必要な確率変数に対して、現在時刻から乱数の種を計算する
pub fn create_seeds_sextet() -> [u32; 6] {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    [
        duration.as_millis() as u32,
        std::u32::MAX - duration.as_nanos() as u32,
        duration.as_secs() as u32,
        std::u32::MAX - duration.as_micros() as u32,
        (duration.as_secs() as u32) / 60_u32,
        (duration.as_millis() as u32) / 60_u32,
    ]
}

/*
// 変数の値が重ならないように変更するマクロ
macro_rules! adjust_values {
    ($($value:expr),*) => {{
        let mut values = [$($value),*];
        for i in 0..(values.len()-1) {
            for j in (i+1)..values.len() {
                if values[i] == values[j] {
                    values[j] = (values[j] << 3) ^ (values[i] >> 2);
                    if values[j] == 0 {
                        values[j] = 1192;
                    }
                }
            }
        }
        values
    }};
}
pub(crate) use adjust_values; // クレート内部でマクロを使用するトリック
*/

// 配列の要素を全て異なる値に変更するマクロ
macro_rules! adjust_seeds {
    ($array:expr) => {{
        let mut copy_array = $array;
        for i in 0..copy_array.len() {
            for j in 0..copy_array.len() {
                if i < j {
                    if copy_array[i] == copy_array[j] {
                        copy_array[j] = (copy_array[j] << 3) ^ (copy_array[i] >> 2);
                        if copy_array[j] == 0 {
                            copy_array[j] = 1192;
                        }
                    }
                }
            }
        }
        copy_array
    }};
}
pub(crate) use adjust_seeds;

// 連続型確率変数

/// 一様乱数
/// # 使用例
/// ```
/// let mut uniform = rand_simple::Uniform::new(1192u32);
/// println!("初期設定の場合、閉区間[0, 1]の一様乱数に従う乱数を返す -> {}", uniform.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let min: f64 = -1f64;
/// let max: f64 = 1f64;
/// let result: Result<(f64, f64), &str> = uniform.try_set_params(min, max);
/// println!("閉区間[{}, {}]の一様乱数を生成する -> {}", min, max, uniform.sample());
/// ```
pub struct Uniform {
    xyzuv: [u32; 5], // 状態変数
    min: f64,        // 最小値
    range: f64,      // 範囲
}

/// 正規分布
/// # 使用例
/// ```
/// let mut normal = rand_simple::Normal::new([1192u32, 765u32]);
/// println!("平均値 μ = 0, 分散 σ^2 = 1 の標準正規分布乱数を生成する -> {}", normal.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let mean: f64 = -3f64;
/// let variance: f64 = 2f64;
/// let result: Result<(f64, f64), &str> = normal.try_set_params(mean, variance);
/// println!("平均値 μ = {}, 分散 σ^2 = {} の正規分布乱数を生成する -> {}", mean, variance, normal.sample());
/// ```
pub struct Normal {
    xyzuv0: [u32; 5], // 状態変数
    xyzuv1: [u32; 5], // 状態変数
    mean: f64,        // 平均
    std: f64,         // 標準偏差
}

/// 半正規分布
/// # 使用例
/// ```
/// let mut half_normal = rand_simple::HalfNormal::new([1192u32, 765u32]);
/// println!("分散 σ^2 = 1 の標準半正規分布乱数を生成する -> {}", half_normal.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let variance: f64 = 2f64;
/// let result: Result<f64, &str> = half_normal.try_set_params(variance);
/// println!("分散 σ^2 = {} の半正規分布乱数を生成する -> {}", variance, half_normal.sample());
/// ```
pub struct HalfNormal {
    xyzuv0: [u32; 5], // 状態変数
    xyzuv1: [u32; 5], // 状態変数
    std: f64,         // 標準偏差
}

/// 対数正規分布
/// # 使用例
/// ```
/// let mut log_normal = rand_simple::LogNormal::new([1192u32, 765u32]);
/// println!("平均値 μ = 0, 分散 σ^2 = 1 の標準対数正規分布乱数を生成する -> {}", log_normal.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let mean: f64 = -3f64;
/// let variance: f64 = 2f64;
/// let result: Result<(f64, f64), &str> = log_normal.try_set_params(mean, variance);
/// println!("平均値 μ = {}, 分散 σ^2 = {} の正規分布乱数を生成する -> {}", mean, variance, log_normal.sample());
/// ```
pub struct LogNormal {
    xyzuv0: [u32; 5], // 状態変数
    xyzuv1: [u32; 5], // 状態変数
    mean: f64,        // 平均
    std: f64,         // 標準偏差
}

/// コーシー分布
/// # 使用例
/// ```
/// let mut cauchy = rand_simple::Cauchy::new([1192u32, 765u32]);
/// println!("位置母数 μ = 0, 尺度母数 θ = 1 の標準コーシー分布に従う乱数を生成する -> {}", cauchy.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let location: f64 = -2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = cauchy.try_set_params(location, scale);
/// println!("位置母数 μ = {}, 尺度母数 θ = {} のコーシー分布に従う乱数を生成する -> {}", location, scale, cauchy.sample());
/// ```
pub struct Cauchy {
    xyzuv0: [u32; 5], // 状態変数
    xyzuv1: [u32; 5], // 状態変数
    location: f64,    // 位置母数
    scale: f64,       // 尺度母数
}

/// 半コーシー分布
/// # 使用例
/// ```
/// let mut half_cauchy = rand_simple::HalfCauchy::new([1192u32, 765u32]);
/// println!("尺度母数 θ = 1 の標準半コーシー分布に従う乱数を生成する -> {}", half_cauchy.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let scale: f64 = 1.5f64;
/// let result: Result<f64, &str> = half_cauchy.try_set_params(scale);
/// println!("尺度母数 θ = {} の半コーシー分布に従う乱数を生成する -> {}", scale, half_cauchy.sample());
/// ```
pub struct HalfCauchy {
    xyzuv0: [u32; 5], // 状態変数
    xyzuv1: [u32; 5], // 状態変数
    scale: f64,       // 尺度母数
}

/// レヴィ分布
/// # 使用例
/// ```
/// let mut levy = rand_simple::Levy::new([1192u32, 765u32]);
/// println!("位置母数 μ = 0, 尺度母数 θ = 1 の標準レヴィ分布に従う乱数を生成する -> {}", levy.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let location: f64 = -2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = levy.try_set_params(location, scale);
/// println!("位置母数 μ = {}, 尺度母数 θ = {} のレヴィ分布に従う乱数を生成する -> {}", location, scale, levy.sample());
/// ```
pub struct Levy {
    xyzuv0: [u32; 5], // 状態変数
    xyzuv1: [u32; 5], // 状態変数
    location: f64,    // 位置母数
    scale: f64,       // 尺度母数
}

/// 指数分布
/// # 使用例
/// ```
/// let mut exponential = rand_simple::Exponential::new(1192u32);
/// println!("尺度母数 θ = 1の標準指数分布に従う乱数を生成する -> {}", exponential.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let scale: f64 = 1.5f64;
/// let result: Result<f64, &str> = exponential.try_set_params(scale);
/// println!("尺度母数 θ = {} の指数分布に従う乱数を生成する -> {}", scale, exponential.sample());
/// ```
pub struct Exponential {
    xyzuv: [u32; 5],         // 状態変数
    previous_uniform_1: f64, // 前回使用した一様乱数
    scale: f64,              // 尺度母数
}

/// ラプラス分布
/// # 使用例
/// ```
/// let mut laplace = rand_simple::Laplace::new(1192u32);
/// println!("位置母数 μ = 0, 尺度母数 θ = 1 の標準ラプラス分布に従う乱数を生成する -> {}", laplace.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let location: f64 = -2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = laplace.try_set_params(location, scale);
/// println!("位置母数 μ = {}, 尺度母数 θ = {} のラプラス分布に従う乱数を生成する -> {}", location, scale, laplace.sample());
/// ```
pub struct Laplace {
    xyzuv: [u32; 5],         // 状態変数
    previous_uniform_1: f64, // 前回使用した一様乱数
    location: f64,           // 位置母数
    scale: f64,              // 尺度母数
}

/// 対数ラプラス分布
/// # 使用例
/// ```
/// let mut log_laplace = rand_simple::Laplace::new(1192u32);
/// println!("位置母数 μ = 0, 尺度母数 θ = 1 の標準対数ラプラス分布に従う乱数を生成する -> {}", log_laplace.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let location: f64 = -2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = log_laplace.try_set_params(location, scale);
/// println!("位置母数 μ = {}, 尺度母数 θ = {} の対数ラプラス分布に従う乱数を生成する -> {}", location, scale, log_laplace.sample());
/// ```
pub struct LogLaplace {
    xyzuv: [u32; 5],         // 状態変数
    previous_uniform_1: f64, // 前回使用した一様乱数
    location: f64,           // 位置母数
    scale: f64,              // 尺度母数
}

/// レイリー分布
/// # 使用例
/// ```
/// let mut rayleigh = rand_simple::Rayleigh::new(1192u32);
/// println!("尺度母数 θ = 1の標準レイリー分布に従う乱数を生成する -> {}", rayleigh.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let scale: f64 = 1.5f64;
/// let result: Result<f64, &str> = rayleigh.try_set_params(scale);
/// println!("尺度母数 θ = {} の標準レイリー分布に従う乱数を生成する -> {}", scale, rayleigh.sample());
/// ```
pub struct Rayleigh {
    xyzuv: [u32; 5],         // 状態変数
    previous_uniform_1: f64, // 前回使用した一様乱数
    scale: f64,              // 尺度母数
}

/// ワイブル分布
/// # 使用例
/// ```
/// let mut weibull = rand_simple::Weibull::new(1192u32);
/// println!("形状母数 γ = 1, 尺度母数 η = 1 の標準ワイブル分布に従う乱数を生成する -> {}", weibull.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let shape: f64 = 2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = weibull.try_set_params(shape, scale);
/// println!("形状母数 γ = {}, 尺度母数 η = {} のワイブル分布に従う乱数を生成する -> {}", shape, scale, weibull.sample());
/// ```
pub struct Weibull {
    xyzuv: [u32; 5],         // 状態変数
    previous_uniform_1: f64, // 前回使用した一様乱数
    shape_inv: f64,          // 形状母数の逆数
    scale: f64,              // 尺度母数
}

/// 反射ワイブル分布
/// # 使用例
/// ```
/// let mut reflected_weibull = rand_simple::ReflectedWeibull::new(1192u32);
/// println!("形状母数 γ = 1, 位置母数 μ = 0, 尺度母数 η = 1 の標準反射ワイブル分布に従う乱数を生成する -> {}", reflected_weibull.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let shape: f64 = 2f64;
/// let location: f64 = 3f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64, f64), &str> = reflected_weibull.try_set_params(shape, location, scale);
/// println!("形状母数 γ = {}, 位置母数 μ = {}, 尺度母数 η = {} の反射ワイブル分布に従う乱数を生成する -> {}", shape, location, scale, reflected_weibull.sample());
/// ```
pub struct ReflectedWeibull {
    xyzuv: [u32; 5],         // 状態変数
    previous_uniform_1: f64, // 前回使用した一様乱数
    shape_inv: f64,          // 形状母数の逆数
    location: f64,           // 位置母数
    scale: f64,              // 尺度母数
}

/// フレシェ分布
/// # 使用例
/// ```
/// let mut frechet = rand_simple::Frechet::new(1192u32);
/// println!("形状母数 γ = 1, 尺度母数 η = 1 の標準フレシェ分布に従う乱数を生成する -> {}", frechet.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let shape: f64 = 2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = frechet.try_set_params(shape, scale);
/// println!("形状母数 γ = {}, 尺度母数 η = {} のフレシェ分布に従う乱数を生成する -> {}", shape, scale, frechet.sample());
/// ```
pub struct Frechet {
    xyzuv: [u32; 5],         // 状態変数
    previous_uniform_1: f64, // 前回使用した一様乱数
    shape_inv: f64,          // 形状母数の逆数
    scale: f64,              // 尺度母数
}

/// ガンベル分布
/// # 使用例
/// ```
/// let mut gunbel = rand_simple::Gunbel::new(1192u32);
/// println!("位置母数 μ = 0, 尺度母数 η = 1 の標準反射ワイブル分布に従う乱数を生成する -> {}", gunbel.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let location: f64 = 3f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = gunbel.try_set_params(location, scale);
/// println!("位置母数 μ = {}, 尺度母数 η = {} の反射ワイブル分布に従う乱数を生成する -> {}", location, scale, gunbel.sample());
/// ```
pub struct Gunbel {
    xyzuv: [u32; 5],         // 状態変数
    previous_uniform_1: f64, // 前回使用した一様乱数
    location: f64,           // 位置母数
    scale: f64,              // 尺度母数
}

/// ガンマ分布
/// # 使用例
/// ```
/// let mut gamma = rand_simple::Gamma::new([1192u32, 765u32, 1543u32]);
/// println!("形状母数 α = 1, 尺度母数 β = 1 の標準ガンマ分布に従う乱数を生成する -> {}", gamma.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let shape: f64 = 2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = gamma.try_set_params(shape, scale);
/// println!("形状母数 α = {}, 尺度母数 β = {} のガンマ分布に従う乱数を生成する -> {}", shape, scale, gamma.sample());
/// ```
pub struct Gamma {
    xyzuv: [u32; 5],         // 状態変数
    previous_uniform_1: f64, // 前回使用した一様乱数
    xyzuv0: [u32; 5],        // 状態変数
    xyzuv1: [u32; 5],        // 状態変数
    shape: f64,
    scale: f64,
}

/// ベータ分布
/// # 使用例
/// ```
/// let mut beta = rand_simple::Beta::new([1192u32, 765u32, 1543u32, 2003u32, 1867u32, 1688u32]);
/// println!("形状母数 α = 1, 形状母数 β = 1 の標準ベータ分布に従う乱数を生成する -> {}", beta.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let shape_alpha: f64 = 2f64;
/// let shape_beta: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = beta.try_set_params(shape_alpha, shape_beta);
/// println!("形状母数 α = {}, 形状母数 β = {} のベータ分布に従う乱数を生成する -> {}", shape_alpha, shape_beta, beta.sample());
/// ```
pub struct Beta {
    xyzuv_alpha: [u32; 5],         // 状態変数
    previous_uniform_1_alpha: f64, // 前回使用した一様乱数
    xyzuv0_alpha: [u32; 5],        // 状態変数
    xyzuv1_alpha: [u32; 5],        // 状態変数
    shape_alpha: f64,

    xyzuv_beta: [u32; 5],         // 状態変数
    previous_uniform_1_beta: f64, // 前回使用した一様乱数
    xyzuv0_beta: [u32; 5],        // 状態変数
    xyzuv1_beta: [u32; 5],        // 状態変数
    shape_beta: f64,
}

// ディリクレ分布
//pub struct Dirichlet {}

/// べき関数分布
/// # 使用例
/// ```
/// let mut power_function = rand_simple::PowerFunction::new(1192u32);
/// println!("初期設定の場合、閉区間[0, 1]の一様乱数に従う乱数を返す -> {}", power_function.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let shape: f64 = 2_f64;
/// let min: f64 = -1f64;
/// let max: f64 = 1f64;
/// let result: Result<(f64, f64, f64), &str> = power_function.try_set_params(shape,min, max);
/// println!("形状母数 {}, 閉区間[{}, {}]の一様乱数を生成する -> {}", shape, min, max, power_function.sample());
/// ```
pub struct PowerFunction {
    xyzuv: [u32; 5],  // 状態変数
    shape_gamma: f64, // 形状母数
    min_a: f64,       // 境界母数(小範)
    max_b: f64,       // 境界母数(大範)
}

// 指数べき分布
//pub struct ExponentialPower {}

/// アーラン分布
/// # 使用例
/// ```
/// let mut erlang = rand_simple::Erlang::new([1192u32, 765u32, 1543u32]);
/// println!("形状母数 r = 1, 尺度母数 θ = 1 の標準アーラン分布に従う乱数を生成する -> {}", erlang.sample());
///
/// // 確率変数のパラメータを変更する場合
/// let shape: i64 = 2i64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(i64, f64), &str> = erlang.try_set_params(shape, scale);
/// println!("形状母数 r = {}, 尺度母数 θ = {} のアーラン分布に従う乱数を生成する -> {}", shape, scale, erlang.sample());
/// ```
pub struct Erlang {
    xyzuv: [u32; 5],         // 状態変数
    previous_uniform_1: f64, // 前回使用した一様乱数
    xyzuv0: [u32; 5],        // 状態変数
    xyzuv1: [u32; 5],        // 状態変数
    shape: f64,
    scale: f64,
}

// ガンマ二乗分布
//pub struct ChiSquare {}

// ガンマ分布
//pub struct Chi {}

// F分布
//pub struct FDistribution {}

// t分布
//pub struct TDistribution {}

// 逆ガウス分布
//pub struct InverseGaussian {}

// 三角分布
//pub struct Triangular {}

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
