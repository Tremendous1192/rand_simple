#![doc = include_str!("../README.md")]


mod distributions; // 確率変数の詳細
#[cfg(test)] mod test_distributions; // 機能確認のためのテストモジュール
#[cfg(test)] mod sandbox; // 試行錯誤するためのテストモジュール
use std::cell::Cell; // 書き換え可能なメンバー変数
use std::time::{SystemTime, UNIX_EPOCH}; // 時刻の取得

// 共通処理
// 状態変数(x, y, z, w)を設定する
// 下記の論文の初期値を参考にする
// https://www.researchgate.net/publication/5142825_Xorshift_RNGs
pub(crate) fn initialize(_seed: u32) -> (u32, u32, u32, u32) {
    (123456789, 362436069, 521288629, _seed)
}

// 共通処理
// 閉区間[0, 1]の一様乱数を計算して、状態変数を更新する
// Wikipediaが分かりやすい
// https://ja.wikipedia.org/wiki/Xorshift
pub(crate) fn update(x: &Cell<u32>, y: &Cell<u32>, z: &Cell<u32>, w: &Cell<u32>) -> f64 {
    // t = x ^ (x << 11), x_new = y, y_new = z, z_new = w
    let calculate_t = |arg: u32| arg ^ (arg << 11);
    let t: u32 = calculate_t(x.replace( y.replace( z.replace(w.get()) ) ));

    // w_ new = w ^ (w >> 19) ^ (t ^ (t >>8))
    let calculate_w = |arg: u32| (arg ^ (arg >> 19)) ^ (t ^ (t >> 8));
    w.set( calculate_w(w.take()) );

    (w.get() as f64) / MAX_U32_AS_F64
}

// 一様乱数を計算するための分母
const MAX_U32_AS_F64: f64 = std::u32::MAX as f64;

// 共通処理
/// 現在時刻から乱数の種を計算する関数
pub fn create_seed() -> u32 {
    // 4_294_967_295u32 / 24 * 60 * 60 * 1000ミリ秒/日 ≒ 49.7日周期
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() as u32
}

// 共通処理
/// 正規分布等2つの乱数の種が必要な確率変数に対して、現在時刻から乱数の種を計算する
pub fn create_seeds() -> (u32, u32) {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    // 49.7日周期と4秒周期の組み合わせ
    (duration.as_millis() as u32, std::u32::MAX - duration.as_nanos() as u32)
}

// 連続型確率変数

/// 一様乱数を計算する構造体
/// # 使用例
/// ```
/// let uniform = rand_simple::Uniform::new(1192u32);
/// 
/// // 初期設定の場合、閉区間[0, 1]の一様乱数に従う乱数を返す
/// assert_eq!(uniform.sample(), 0.8512317447111084f64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let min: f64 = -1f64;
/// let max: f64 = 1f64;
/// let result: Result<(f64, f64), &str> = uniform.try_set_params(min, max);
/// assert_eq!(uniform.sample(), -0.7648924006533093f64);
/// ```
pub struct Uniform {
	x: Cell<u32>, y: Cell<u32>, z: Cell<u32>, w: Cell<u32>, // 状態変数
    min: Cell<f64>, // 最小値
    range: Cell<f64>, // 範囲
}

/// 正規分布を計算する構造体
/// # 使用例
/// ```
/// let normal = rand_simple::Normal::new(1192u32, 765u32);
/// 
/// // 初期設定の場合、平均値 0, 標準偏差 1 の標準正規分布に従う乱数を返す
/// assert_eq!(normal.sample(), 0.11478775584530312f64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let mean: f64 = -3f64;
/// let variance: f64 = 2f64;
/// let result: Result<(f64, f64), &str> = normal.try_set_params(mean, variance);
/// assert_eq!(normal.sample(), 0.11478778909773256f64);
/// ```
pub struct Normal {
    x0: Cell<u32>, y0: Cell<u32>, z0: Cell<u32>, w0: Cell<u32>, // 状態変数
	x1: Cell<u32>, y1: Cell<u32>, z1: Cell<u32>, w1: Cell<u32>, // 状態変数
    even_flag: Cell<bool>, // 乱数計算が偶数回目かどうかのフラグ
    even_result: Cell<f64>, // 偶数回目の計算結果
    mean: Cell<f64>, // 平均
    std: Cell<f64>, // 標準偏差
}

/// 半正規分布を計算する構造体
/// # 使用例
/// ```
/// let half_normal = rand_simple::HalfNormal::new(1192u32, 765u32);
/// 
/// // 初期設定の場合、標準偏差 1 の標準半正規分布に従う乱数を返す
/// assert_eq!(half_normal.sample(), 1.8943489630074781f64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let variance: f64 = 2f64;
/// let result: Result<f64, &str> = half_normal.try_set_params(variance);
/// assert_eq!(half_normal.sample(), 1.8943544071672804f64);
/// ```
pub struct HalfNormal {
    x0: Cell<u32>, y0: Cell<u32>, z0: Cell<u32>, w0: Cell<u32>, // 状態変数
	x1: Cell<u32>, y1: Cell<u32>, z1: Cell<u32>, w1: Cell<u32>, // 状態変数
    even_flag: Cell<bool>, // 乱数計算が偶数回目かどうかのフラグ
    even_result: Cell<f64>, // 偶数回目の計算結果
    std: Cell<f64>, // 標準偏差
}

// 対数正規分布を計算する構造体
//pub struct LogNormal {}

/// コーシー分布を計算する構造体
/// # 使用例
/// ```
/// let cauchy = rand_simple::Cauchy::new(1192u32, 765u32);
/// 
/// // 初期設定の場合、位置母数 μ = 0, 尺度母数 θ = 1の標準コーシー分布に従う乱数を返す
/// assert_eq!(cauchy.sample(), 0.9999997103138784f64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let location: f64 = -2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = cauchy.try_set_params(location, scale);
/// assert_eq!(cauchy.sample(), -0.49999188999688693f64);
/// ```
pub struct Cauchy {
    x0: Cell<u32>, y0: Cell<u32>, z0: Cell<u32>, w0: Cell<u32>, // 状態変数
	x1: Cell<u32>, y1: Cell<u32>, z1: Cell<u32>, w1: Cell<u32>, // 状態変数
    location: Cell<f64>, // 位置母数
    scale: Cell<f64>, // 尺度母数
}

/// 半コーシー分布を計算する構造体
/// # 使用例
/// ```
/// let half_cauchy = rand_simple::HalfCauchy::new(1192u32, 765u32);
/// 
/// // 初期設定の場合、尺度母数 θ = 1の標準半コーシー分布に従う乱数を返す
/// assert_eq!(half_cauchy.sample(), 0.9999971261133705f64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let scale: f64 = 1.5f64;
/// let result: Result<f64, &str> = half_cauchy.try_set_params(scale);
/// assert_eq!(half_cauchy.sample(), 1.500000918541327f64);
/// ```
pub struct HalfCauchy {
    x0: Cell<u32>, y0: Cell<u32>, z0: Cell<u32>, w0: Cell<u32>, // 状態変数
	x1: Cell<u32>, y1: Cell<u32>, z1: Cell<u32>, w1: Cell<u32>, // 状態変数
    scale: Cell<f64>, // 尺度母数
}

/// レヴィ分布を計算する構造体
/// # 使用例
/// ```
/// let levy = rand_simple::Levy::new(1192u32, 765u32);
/// 
/// // 初期設定の場合、位置母数 μ = 0, 尺度母数 θ = 1の標準レヴィ分布に従う乱数を返す
/// assert_eq!(levy.sample(), 0.27866346364478645f64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let location: f64 = -2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = levy.try_set_params(location, scale);
/// assert_eq!(levy.sample(), 0.2786618619526834f64);
/// ```
pub struct Levy {
    x0: Cell<u32>, y0: Cell<u32>, z0: Cell<u32>, w0: Cell<u32>, // 状態変数
	x1: Cell<u32>, y1: Cell<u32>, z1: Cell<u32>, w1: Cell<u32>, // 状態変数
    even_flag: Cell<bool>, // 乱数計算が偶数回目かどうかのフラグ
    even_result: Cell<f64>, // 偶数回目の計算結果
    location: Cell<f64>, // 位置母数
    scale: Cell<f64>, // 尺度母数
}

/// 指数分布を計算する構造体
/// # 使用例
/// ```
/// let exponential = rand_simple::Exponential::new(1192u32);
/// 
/// // 初期設定の場合、尺度母数 θ = 1の標準指数分布に従う乱数を返す
/// assert_eq!(exponential.sample(), 1.9053655174552453f64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let scale: f64 = 1.5f64;
/// let result: Result<f64, &str> = exponential.try_set_params(scale);
/// assert_eq!(exponential.sample(), 0.187586182253475f64);
/// ```
pub struct Exponential {
    x: Cell<u32>, y: Cell<u32>, z: Cell<u32>, w: Cell<u32>, // 状態変数
    scale: Cell<f64>, // 尺度母数
}

/// ラプラス分布を計算する構造体
/// # 使用例
/// ```
/// let laplace = rand_simple::Laplace::new(1192u32);
/// 
/// // 初期設定の場合、位置母数 μ = 0, 尺度母数 θ = 1の標準ラプラス分布に従う乱数を返す
/// assert_eq!(laplace.sample(), -0.824946373682539f64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let location: f64 = -2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = laplace.try_set_params(location, scale);
/// assert_eq!(laplace.sample(), -1.4491717380062097f64);
/// ```
pub struct Laplace {
    x: Cell<u32>, y: Cell<u32>, z: Cell<u32>, w: Cell<u32>, // 状態変数
    location: Cell<f64>, // 位置母数
    scale: Cell<f64>, // 尺度母数
}

/// レイリー分布を計算する構造体
/// # 使用例
/// ```
/// let rayleigh = rand_simple::Rayleigh::new(1192u32);
/// 
/// // 初期設定の場合、尺度母数 σ = 1の標準指数分布に従う乱数を返す
/// assert_eq!(rayleigh.sample(), 1.742465812716269f64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let scale: f64 = 1.5f64;
/// let result: Result<f64, &str> = rayleigh.try_set_params(scale);
/// assert_eq!(rayleigh.sample(), 1.5446111320492815f64);
/// ```
pub struct Rayleigh {
    x: Cell<u32>, y: Cell<u32>, z: Cell<u32>, w: Cell<u32>, // 状態変数
    scale: Cell<f64>, // 尺度母数
}

// ワイブル分布を計算する構造体
/// # 使用例
/// ```
/// let weibull = rand_simple::Weibull::new(1192u32);
/// 
/// // 初期設定の場合、形状母数 γ = 1, 尺度母数 η = 1の標準ワイブル分布に従う乱数を返す
/// assert_eq!(weibull.sample(), 1.9053655174552453f64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let shape: f64 = 2f64;
/// let scale: f64 = 1.5f64;
/// let result: Result<(f64, f64), &str> = weibull.try_set_params(shape, scale);
/// assert_eq!(weibull.sample(), 0.530451951999625f64);
/// ```
pub struct Weibull {
    x: Cell<u32>, y: Cell<u32>, z: Cell<u32>, w: Cell<u32>, // 状態変数
    shape_inv: Cell<f64>, // 形状母数の逆数
    scale: Cell<f64>, // 尺度母数
}

// ガンベル分布を計算する構造体
//pub struct Gunbel {}

// ガンマ分布を計算する構造体
//pub struct Gamma {}

// ベータ分布を計算する構造体
//pub struct Beta {}

// ディリクレ分布を計算する構造体
//pub struct Dirichlet {}

// べき関数分布を計算する構造体
//pub struct PowerFunction {}

// 指数べき分布を計算する構造体
//pub struct ExponentialPower {}

// アーラン分布を計算する構造体
//pub struct Erlang {}

// ガンマ二乗分布を計算する構造体
//pub struct ChiSquare {}

// ガンマ分布を計算する構造体
//pub struct Chi {}

// F分布を計算する構造体
//pub struct FDistribution {}

// t分布を計算する構造体
//pub struct TDistribution {}

// 逆ガウス分布を計算する構造体
//pub struct InverseGaussian {}

// 三角分布を計算する構造体
//pub struct Triangular {}

// パレート分布を計算する構造体
//pub struct Pareto {}

// ロジスティック分布を計算する構造体
//pub struct Logistic {}

// 双曲線正割分布を計算する構造体
//pub struct HeyperbolicSecant {}

// 余弦分布を計算する構造体
//pub struct RaisedCosine {}

// 逆正弦分布を計算する構造体
//pub struct Arcsine {}

// フォン・ミーゼス分布を計算する構造体
//pub struct VonMises {}

// 非心ガンマ分布を計算する構造体
//pub struct NonCentralGamma {}

// 非心ベータ分布を計算する構造体
//pub struct NonCentralBeta {}

// 非心ガンマ二乗分布を計算する構造体
//pub struct NonCentralChiSquare {}

// 非心ガンマ分布を計算する構造体
//pub struct NonCentralChi {}

// 非心F分布を計算する構造体
//pub struct NonCentralF {}

// 非心t分布を計算する構造体
//pub struct NonCentralT {}

// プランク分布を計算する構造体
//pub struct Plank {}


// 離散型確率変数

/// ベルヌーイ分布を計算する構造体
/// # 使用例
/// ```
/// let bernoulli = rand_simple::Bernoulli::new(1192u32);
/// 
/// // 初期設定の場合、発生確率 0.5の事象が生じたか(1u64)、否か(0u64)を返す
/// assert_eq!(bernoulli.sample(), 0u64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let probability: f64 = 0.8f64;
/// let result: Result<f64, &str> = bernoulli.try_set_params(probability);
/// assert_eq!(bernoulli.sample(), 1u64);
/// ```
pub struct Bernoulli {
    x: Cell<u32>, y: Cell<u32>, z: Cell<u32>, w: Cell<u32>, // 状態変数
    probability: Cell<f64>, // 発生確率
}

// 二項分布
//pub  struct Binomial {}

/// 幾何分布を計算する構造体
/// # 使用例
/// ```
/// let geometric = rand_simple::Geometric::new(1192u32);
/// 
/// // 初期設定の場合、発生確率 0.5の事象が初めて生じた試行回数を返す
/// assert_eq!(geometric.sample(), 2u64);
/// 
/// // 確率変数のパラメータを変更する場合
/// let probability: f64 = 0.8f64;
/// let result: Result<f64, &str> = geometric.try_set_params(probability);
/// assert_eq!(geometric.sample(), 1u64);
/// ```
pub struct Geometric {
    x: Cell<u32>, y: Cell<u32>, z: Cell<u32>, w: Cell<u32>, // 状態変数
    probability: Cell<f64>, // 発生確率
}

// ポアソン分布を計算する構造体
//pub struct Poisson {}

// 超幾何分布を計算する構造体
//pub struct HeyperGeometric {}

// 多項分布を計算する構造体
//pub struct Multinominal {}

// 負の二項分布を計算する構造体
//pub struct NegativeBinomial {}

// 負の超幾何分布を計算する構造体
//pub struct NegativeHeyperGeometric {}

// 対数級数分布を計算する構造体
//pub struct LogarithmicSeries {}

// ユール・シモン分布を計算する構造体
//pub struct YuleSimon {}

// ジップ・マンデルブロート分布を計算する構造体
//pub struct ZipfMandelbrot {}

// ゼータ分布を計算する構造体
//pub struct Zeta {}