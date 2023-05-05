//! このライブラリは、疑似乱数を簡単に呼び出すことができるライブラリです。
//! 
//! 例えば、```use rand_simple::Uniform;```と宣言するだけで、一様分布乱数を使用できます。
//! 
//! 偉大な先達[rand](https://crates.io/crates/rand)と比較して、
//! 簡素なモジュール宣言と豊富な確率変数による使いやすさを目指しています。
//! # 使用例
//! ```
//! use rand_simple::Uniform;
//! let uniform = Uniform::new(1192u32);
//! let next = uniform.sample(); // 閉区間[0, 1]の一様乱数
//! println!("乱数: {}", next); // 0.8698977918526851f64
//! ```

//mod macros; // マクロモジュール
mod distributions; // 確率変数の詳細
#[cfg(test)] mod test_distributions; // テストモジュール
use std::cell::Cell; // 書き換え可能なメンバー変数
use std::time::{SystemTime, UNIX_EPOCH}; // 時刻の取得

// 共通処理
// 状態変数(x, y, z, w)を設定する
pub(crate) fn set_state(_seed: u32) -> (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>) {
    let x: u32 = 123456789;
    let y: u32 = (_seed as u64 >> 32) as u32 & 0xFFFFFFFF;
    let z: u32 = _seed & 0xFFFFFFFF;
    let w: u32 = x ^ z;

    (Cell::<u32>::new(x), Cell::<u32>::new(y), Cell::<u32>::new(z), Cell::<u32>::new(w))
}

// 共通処理
// 閉区間[0, 1]の一様乱数を計算して、状態変数を更新する
pub(crate) fn update_and_uniform(_xyzw: &(Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>)) -> f64 {
    // 一様乱数を計算する
    let t: u32 = _xyzw.0.get() ^ (_xyzw.0.get() << 11);
    let x: u32 = _xyzw.1.get();
    let y: u32 = _xyzw.2.get();
    let z: u32 = _xyzw.3.get();
    let mut w: u32 = _xyzw.3.get();
    w = (w ^ (w >> 19)) ^ (t ^ (t >> 8));

    // 状態変数を更新する
    _xyzw.0.set(x);
    _xyzw.1.set(y);
    _xyzw.2.set(z);
    _xyzw.3.set(w);

    // 一様乱数を返す
    (w as f64) / MAX_U32_AS_F64
}

// 一様乱数を計算するための分母
// 一々呼び出すよりは定数にしておいた方が計算時間が短いのではないか?
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
    (duration.as_millis() as u32, duration.as_nanos() as u32)
}

// 連続型確率変数

/// 一様乱数を計算する構造体
/// # 使用例 1 (new関数)
/// ```
/// use rand_simple::Uniform;
/// let uniform = Uniform::new(1192u32);
/// let next = uniform.sample(); // 閉区間[0, 1]の一様乱数
/// println!("乱数: {}", next); // 0.8698977918526851f64
/// ```
/// # 使用例 2 (マクロ・引数有り)
/// ```
/// use rand_simple::create_uniform;
/// let uniform = create_uniform!(1192u32);
/// let next = uniform.sample(); // 閉区間[0, 1]の一様乱数
/// println!("乱数: {}", next); // 0.8698977918526851f64
/// ```
/// # 使用例 3 (マクロ・引数無し)
/// ```
/// use rand_simple::create_uniform;
/// let uniform = create_uniform!();
/// let next = uniform.sample(); // 閉区間[0, 1]の一様乱数
/// println!("乱数: {}", next); // 値不明
/// ```
pub struct Uniform {
    xyzw: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>) // 状態変数
}
/// オーバーロードを付与するためのテストトレイト
/// 
/// 運用がうまくいくことを確認した後、本番トレイトに置き換えて削除する
pub trait TestUniformSample {
    fn test_sample(uniform: &Uniform, _foo: &Self) -> f64;
}

/// 正規分布を計算する構造体
/// # 使用例 1 (new関数)
/// ```
/// use rand_simple::Normal;
/// let normal = Normal::new(1192u32, 765u32);
/// let next = normal.sample(); // 平均値 0, 標準偏差 1 の標準正規分布
/// println!("乱数: {}", next); // -1.2296205447119757
/// ```
/// # 使用例 2 (マクロ・引数有り)
/// ```
/// use rand_simple::create_normal;
/// let create_normal = create_normal!(1192u32, 765u32);
/// let next = create_normal.sample(); // 平均値 0, 標準偏差 1 の標準正規分布
/// println!("乱数: {}", next); // -1.2296205447119757
/// ```
/// # 使用例 3 (マクロ・引数無し)
/// ```
/// use rand_simple::create_normal;
/// let create_normal = create_normal!();
/// let next = create_normal.sample(); // 平均値 0, 標準偏差 1 の標準正規分布
/// println!("乱数: {}", next); // 値不明
/// ```
pub struct Normal {
    xyzw_1: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>), // 状態変数
    xyzw_2: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>), // 状態変数
    even_flag: Cell<bool>, // 乱数計算が偶数回目かどうかのフラグ
    even_result: Cell<f64>, // 偶数回目の計算結果
}

/// 半正規分布を計算する構造体
/// # 使用例 1 (new関数)
/// ```
/// use rand_simple::HalfNormal;
/// let half_normal = HalfNormal::new(1192u32, 765u32);
/// let next = half_normal.sample(); // 標準偏差 1 の標準半正規分布
/// println!("乱数: {}", next); // 2.5308912695634582
/// ```
/// # 使用例 2 (マクロ・引数有り)
/// ```
/// use rand_simple::create_half_normal;
/// let half_normal = create_half_normal!(1192u32, 765u32);
/// let next = half_normal.sample(); // 標準偏差 1 の標準半正規分布
/// println!("乱数: {}", next); // 2.5308912695634582
/// ```
/// # 使用例 3 (マクロ・引数無し)
/// ```
/// use rand_simple::create_half_normal;
/// let half_normal = create_half_normal!();
/// let next = half_normal.sample(); // 標準偏差 1 の標準半正規分布
/// println!("乱数: {}", next); // 値不明
/// ```
pub struct HalfNormal {
    xyzw_1: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>), // 状態変数
    xyzw_2: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>), // 状態変数
    even_flag: Cell<bool>, // 乱数計算が偶数回目かどうかのフラグ
    even_result: Cell<f64>, // 偶数回目の計算結果
}

// 対数正規分布を計算する構造体
//pub struct LogNormal {}

/// コーシー分布を計算する構造体
/// # 使用例 1 (new関数)
/// ```
/// use rand_simple::Cauchy;
/// let cauchy = Cauchy::new(1192u32, 765u32);
/// let next = cauchy.sample(); // 位置母数 μ = 0, 尺度母数 θ = 1の標準コーシー分布
/// println!("乱数: {}", next); // 1.0046339315561652f64
/// ```
/// # 使用例 2 (マクロ・引数有り)
/// ```
/// use rand_simple::create_cauchy;
/// let cauchy = create_cauchy!(1192u32, 765u32);
/// let next = cauchy.sample(); // 位置母数 μ = 0, 尺度母数 θ = 1の標準コーシー分布
/// println!("乱数: {}", next); // 1.0046339315561652f64
/// ```
/// # 使用例 3 (マクロ・引数無し)
/// ```
/// use rand_simple::create_cauchy;
/// let cauchy = create_cauchy!();
/// let next = cauchy.sample(); // 位置母数 μ = 0, 尺度母数 θ = 1の標準コーシー分布
/// println!("乱数: {}", next); // 値不明
/// ```
pub struct Cauchy {
    xyzw_1: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>), // 状態変数
    xyzw_2: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>), // 状態変数
}

/// 半コーシー分布を計算する構造体
/// # 使用例 1 (new関数)
/// ```
/// use rand_simple::HalfCauchy;
/// let half_cauchy = HalfCauchy::new(1192u32, 765u32);
/// let next = half_cauchy.sample(); // 尺度母数 θ = 1の標準半コーシー分布
/// println!("乱数: {}", next); // 0.9999951805774843f64
/// ```
/// # 使用例 2 (マクロ・引数有り)
/// ```
/// use rand_simple::create_half_cauchy;
/// let half_cauchy = create_half_cauchy!(1192u32, 765u32);
/// let next = half_cauchy.sample(); // 尺度母数 θ = 1の標準半コーシー分布
/// println!("乱数: {}", next); // 0.9999951805774843f64
/// ```
/// # 使用例 3 (マクロ・引数無し)
/// ```
/// use rand_simple::create_half_cauchy;
/// let half_cauchy = create_half_cauchy!();
/// let next = half_cauchy.sample(); // 尺度母数 θ = 1の標準半コーシー分布
/// println!("乱数: {}", next); // 値不明
/// ```
pub struct HalfCauchy {
    xyzw_1: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>), // 状態変数
    xyzw_2: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>), // 状態変数
}

/// レヴィ分布を計算する構造体
/// # 使用例 1 (new関数)
/// ```
/// use rand_simple::Levy;
/// let levy = Levy::new(1192u32, 765u32);
/// let next = levy.sample(); // 位置母数 μ = 0, 尺度母数 θ = 1の標準レヴィ分布
/// println!("乱数: {}", next); // 0.15611801640551176f64
/// ```
/// # 使用例 2 (マクロ・引数有り)
/// ```
/// use rand_simple::create_levy;
/// let levy = create_levy!(1192u32, 765u32);
/// let next = levy.sample(); // 位置母数 μ = 0, 尺度母数 θ = 1の標準レヴィ分布
/// println!("乱数: {}", next); // 0.15611801640551176f64
/// ```
/// # 使用例 3 (マクロ・引数無し)
/// ```
/// use rand_simple::create_levy;
/// let levy = create_levy!();
/// let next = levy.sample(); // 位置母数 μ = 0, 尺度母数 θ = 1の標準レヴィ分布
/// println!("乱数: {}", next); // 値不明
/// ```
pub struct Levy {
    xyzw_1: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>), // 状態変数
    xyzw_2: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>), // 状態変数
    even_flag: Cell<bool>, // 乱数計算が偶数回目かどうかのフラグ
    even_result: Cell<f64>, // 偶数回目の計算結果
}

/// 指数分布を計算する構造体
/// # 使用例 1 (new関数)
/// ```
/// use rand_simple::Exponential;
/// let exponential = Exponential::new(1192u32);
/// let next = exponential.sample(); // 尺度母数 θ = 1の標準指数分布
/// println!("乱数: {}", next); // 1.4145870106554208f64
/// ```
/// # 使用例 2 (マクロ・引数有り)
/// ```
/// use rand_simple::create_exponential;
/// let exponential = create_exponential!(1192u32);
/// let next = exponential.sample(); // 尺度母数 θ = 1の標準指数分布
/// println!("乱数: {}", next); // 1.4145870106554208f64
/// ```
/// # 使用例 3 (マクロ・引数無し)
/// ```
/// use rand_simple::create_exponential;
/// let exponential = create_exponential!();
/// let next = exponential.sample(); // 尺度母数 θ = 1の標準指数分布
/// println!("乱数: {}", next); // 値不明
/// ```
pub struct Exponential {
    xyzw: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>) // 状態変数
}

/// ラプラス分布を計算する構造体
/// # 使用例 1 (new関数)
/// ```
/// use rand_simple::Laplace;
/// let laplace = Laplace::new(1192u32);
/// let next = laplace.sample(); // 位置母数 μ = 0, 尺度母数 θ = 1の標準ラプラス分布
/// println!("乱数: {}", next); // -0.7214398300954756f64
/// ```
/// # 使用例 2 (マクロ・引数有り)
/// ```
/// use rand_simple::create_laplace;
/// let laplace = create_laplace!(1192u32);
/// let next = laplace.sample(); // 位置母数 μ = 0, 尺度母数 θ = 1の標準ラプラス分布
/// println!("乱数: {}", next); // -0.7214398300954756f64
/// ```
/// # 使用例 3 (マクロ・引数無し)
/// ```
/// use rand_simple::create_laplace;
/// let laplace = create_laplace!();
/// let next = laplace.sample(); // 位置母数 μ = 0, 尺度母数 θ = 1の標準ラプラス分布
/// println!("乱数: {}", next); // 値不明
/// ```
pub struct Laplace {
    xyzw: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>) // 状態変数
}

// レイリー分布を計算する構造体
//pub struct Rayleigh {}

// ワイブル分布を計算する構造体
//pub struct Weibull {}

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
/// # 使用例 1 (new関数)
/// ```
/// use rand_simple::Bernoulli;
/// let bernoulli = Bernoulli::new(1192u32);
/// let next = bernoulli.sample(0.5f64); // 発生確率 0.5の事象が生じたか(1)、否か(0)
/// println!("乱数: {}", next); // 0u32
/// ```
/// # 使用例 2 (マクロ・引数有り)
/// ```
/// use rand_simple::create_bernoulli;
/// let bernoulli = create_bernoulli!(1192u32);
/// let next = bernoulli.sample(0.5f64); // 発生確率 0.5の事象が生じたか(1)、否か(0)
/// println!("乱数: {}", next); // 0u32
/// ```
/// # 使用例 3 (マクロ・引数無し)
/// ```
/// use rand_simple::create_bernoulli;
/// let bernoulli = create_bernoulli!();
/// let next = bernoulli.sample(0.5f64); // 発生確率 0.5の事象が生じたか(1)、否か(0)
/// println!("乱数: {}", next); // 値不明
/// ```
pub struct Bernoulli {
    xyzw: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>) // 状態変数
}

// 二項分布
//pub  struct Binomial {}

/// 幾何分布を計算する構造体
/// # 使用例 1 (new関数)
/// ```
/// use rand_simple::Geometric;
/// let geometric = Geometric::new(1192u32);
/// let next = geometric.sample(0.5f64); // 発生確率 0.5の事象が初めて生じた試行回数
/// println!("乱数: {}", next); // 4u32
/// ```
/// # 使用例 2 (マクロ・引数有り)
/// ```
/// use rand_simple::create_geometric;
/// let geometric = create_geometric!(1192u32);
/// let next = geometric.sample(0.5f64); // 発生確率 0.5の事象が初めて生じた試行回数
/// println!("乱数: {}", next); // 4u32
/// ```
/// # 使用例 3 (マクロ・引数無し)
/// ```
/// use rand_simple::create_geometric;
/// let geometric = create_geometric!();
/// let next = geometric.sample(0.5f64); // 発生確率 0.5の事象が初めて生じた試行回数
/// println!("乱数: {}", next); // 値不明
/// ```
pub struct Geometric {
    xyzw: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>) // 状態変数
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