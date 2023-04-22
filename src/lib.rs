//! このライブラリは、疑似乱数を簡単に呼び出すことができるライブラリです。
//! 
//! 例えば、```use rand_simple::Uniform;```と宣言するだけで、一様分布乱数を使用できます。
//! 
//! 偉大な先達[rand](https://crates.io/crates/rand)に対する差別化として、簡素なモジュール宣言による使いやすさを図っています。

use std::cell::Cell;
mod distributions; // 確率変数の詳細

// 共通処理
// 状態変数の設定
pub(crate) fn set_parameters(_seed: u32) -> (u32, u32, u32, u32) {
    let x: u32 = 123456789;
    let y: u32 = (_seed as u64 >> 32) as u32 & 0xFFFFFFFF;
    let z: u32 = _seed & 0xFFFFFFFF;
    let w: u32 = x ^ z;

    (x, y, z, w)
}

// 共通処理
// 閉区間[0, 1]の一様分布の計算と、状態変数の更新
pub(crate) fn calculate_uniform(_x: u32, _y: u32, _z: u32, _w: u32) -> (u32, u32, u32, u32, f64) {
    // 乱数計算
    let t: u32 = _x ^ (_x << 11);
    let x: u32 = _y;
    let y: u32 = _z;
    let z: u32 = _w;
    let mut w: u32 = _w;
    w = (w ^ (w >> 19)) ^ (t ^ (t >> 8));

    (x, y, z, w, (w as f64) / (std::u32::MAX as f64))
}


/// ベルヌーイ分布を計算する構造体
/// # 使用例
/// ```
/// use rand_simple::Bernoulli;
/// let bernoulli = Bernoulli::new(1192u32);
/// println!("乱数の種: {}", bernoulli.get_seed()); // 乱数の種: 1192u32
/// println!("乱数: {}", bernoulli.next_uint(0.5f64)); // 乱数: 0u32
/// ```
pub struct Bernoulli {
    seed: u32, // 乱数の種
    x_cell: Cell<u32>, // 状態変数 その1
    y_cell: Cell<u32>, // 状態変数 その2
    z_cell: Cell<u32>, // 状態変数 その3
    w_cell: Cell<u32>, // 状態変数 その4
}

/// 一様乱数を計算する構造体
/// # 使用例
/// ```
/// use rand_simple::Uniform;
/// let uniform = Uniform::new(1192u32); // コンストラクタ
/// println!("乱数の種: {}", uniform.get_seed()); // 乱数の種: 1192u32
/// println!("乱数: {}", uniform.next_double()); // 乱数: 0.8698977918526851f64
/// ```
pub struct Uniform {
    seed: u32, // 乱数の種
    x_cell: Cell<u32>, // 状態変数 その1
    y_cell: Cell<u32>, // 状態変数 その2
    z_cell: Cell<u32>, // 状態変数 その3
    w_cell: Cell<u32>, // 状態変数 その4
}

