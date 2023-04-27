//! このライブラリは、疑似乱数を簡単に呼び出すことができるライブラリです。
//! 
//! 例えば、```use rand_simple::Uniform;```と宣言するだけで、一様分布乱数を使用できます。
//! 
//! 偉大な先達[rand](https://crates.io/crates/rand)と比較して、簡素なモジュール宣言による使いやすさを目指しています。

use std::cell::Cell; // 書き換え可能なメンバー変数
mod distributions; // 確率変数の詳細
#[cfg(test)]
mod tests; // テストモジュール

// 共通処理
// 状態変数(x, y, z, w)の設定
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

    // 乱数を返す
    (w as f64) / MAX_U32_AS_F64
}

// 一様乱数を計算するための分母
// 一々呼び出すよりは定数にしておいた方が計算速度が軽いのではないか?
const MAX_U32_AS_F64: f64 = std::u32::MAX as f64;


// 連続型確率変数

/// 一様乱数を計算する構造体
/// # 使用例
/// ```
/// use rand_simple::Uniform;
/// let uniform = Uniform::new(1192u32); // コンストラクタ
/// 
/// // 乱数の種を返す -> 1192u32
/// println!("乱数の種: {}", uniform.get_seed());
/// 
/// // 閉区間[0, 1]の乱数を返す -> 0.8698977918526851f64
/// println!("乱数: {}", uniform.sample());
/// ```
pub struct Uniform {
    seed: u32, // 乱数の種
    xyzw: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>) // 状態変数
}

// 正規分布を計算する構造体
//struct Normal {}

// 半正規分布を計算する構造体
//struct HalfNormal {}

// 対数正規分布を計算する構造体
//struct LogNormal {}

// コーシー分布を計算する構造体
//struct Cauchy {}

// レヴィ分布を計算する構造体
//struct Levy {}

// 指数分布を計算する構造体
//struct Exponential {}

// ラプラス分布を計算する構造体
//struct Laplace {}

// レイリー分布を計算する構造体
//struct Rayleigh {}

// ワイブル分布を計算する構造体
//struct Weibull {}

// ガンベル分布を計算する構造体
//struct Gunbel {}

// ガンマ分布を計算する構造体
//struct Gamma {}

// ベータ分布を計算する構造体
//struct Beta {}

// ディリクレ分布を計算する構造体
//struct Dirichlet {}

// べき関数分布を計算する構造体
//struct PowerFunction {}

// 指数べき分布を計算する構造体
//struct ExponentialPower {}

// アーラン分布を計算する構造体
//struct Erlang {}

// ガンマ二乗分布を計算する構造体
//struct ChiSquare {}

// ガンマ分布を計算する構造体
//struct Chi {}

// F分布を計算する構造体
//struct FDistribution {}

// t分布を計算する構造体
//struct TDistribution {}

// 逆ガウス分布を計算する構造体
//struct InverseGaussian {}

// 三角分布を計算する構造体
//struct Triangular {}

// パレート分布を計算する構造体
//struct Pareto {}

// ロジスティック分布を計算する構造体
//struct Logistic {}

// 双曲線正割分布を計算する構造体
//struct HeyperbolicSecant {}

// 余弦分布を計算する構造体
//struct RaisedCosine {}

// 逆正弦分布を計算する構造体
//struct Arcsine {}

// フォン・ミーゼス分布を計算する構造体
//struct VonMises {}

// 非心ガンマ分布を計算する構造体
//struct NonCentralGamma {}

// 非心ベータ分布を計算する構造体
//struct NonCentralBeta {}

// 非心ガンマ二乗分布を計算する構造体
//struct NonCentralChiSquare {}

// 非心ガンマ分布を計算する構造体
///struct NonCentralChi {}

// 非心F分布を計算する構造体
//struct NonCentralF {}

// 非心t分布を計算する構造体
//struct NonCentralT {}

// プランク分布を計算する構造体
//struct Plank {}


// 離散型確率変数

/// ベルヌーイ分布を計算する構造体
/// # 使用例
/// ```
/// use rand_simple::Bernoulli;
/// let bernoulli = Bernoulli::new(1192u32); // コンストラクタ
/// 
/// // 乱数の種を返す -> 1192u32
/// println!("乱数の種: {}", bernoulli.get_seed());
/// 
/// // 乱数を返す -> 0u32
/// println!("乱数: {}", bernoulli.sample(0.5f64));
/// ```
pub struct Bernoulli {
    seed: u32, // 乱数の種
    xyzw: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>) // 状態変数
}

// 二項分布
// struct Binomial {}

/// 幾何分布を計算する構造体
/// # 使用例
/// ```
/// use rand_simple::Geometric;
/// let geometric = Geometric::new(1192u32); // コンストラクタ
/// 
/// // 乱数の種を返す -> 1192u32
/// println!("乱数の種: {}", geometric.get_seed());
/// 
/// // 乱数を返す -> 4u32
/// println!("乱数: {}", geometric.sample(0.5f64));
/// ```
pub struct Geometric {
    seed: u32, // 乱数の種
    xyzw: (Cell<u32>, Cell<u32>, Cell<u32>, Cell<u32>) // 状態変数
}

// ポアソン分布を計算する構造体
//struct Poisson {}

// 超幾何分布を計算する構造体
//struct HeyperGeometric {}

// 多項分布を計算する構造体
//struct Multinominal {}

// 負の二項分布を計算する構造体
//struct NegativeBinomial {}

// 負の超幾何分布を計算する構造体
//struct NegativeHeyperGeometric {}

// 対数級数分布を計算する構造体
//struct LogarithmicSeries {}

// ユール・シモン分布を計算する構造体
//struct YuleSimon {}

// ジップ・マンデルブロート分布を計算する構造体
//struct ZipfMandelbrot {}

// ゼータ分布を計算する構造体
//struct Zeta {}