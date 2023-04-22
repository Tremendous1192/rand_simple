use std::cell::Cell;
mod distributions; // 確率変数の詳細

// 共通処理
// 一様分布の計算に用いる状態変数を設定する
pub(crate) fn set_parameters(_seed: u32) -> (u32, u32, u32, u32) {
    let x: u32 = 123456789;
    let y: u32 = (_seed as u64 >> 32) as u32 & 0xFFFFFFFF;
    let z: u32 = _seed & 0xFFFFFFFF;
    let w: u32 = x ^ z;

    (x, y, z, w)
}

// 共通処理
// 閉区間[0, 1]の一様分布を計算する
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

/// ベルヌーイ分布
pub struct Bernoulli {
    seed: u32, // 乱数の種
    x_cell: Cell<u32>, // 状態変数 その1
    y_cell: Cell<u32>, // 状態変数 その2
    z_cell: Cell<u32>, // 状態変数 その3
    w_cell: Cell<u32>, // 状態変数 その4
}

/// 閉区間[0, 1]の一様乱数
pub struct Uniform {
    seed: u32, // 乱数の種
    x_cell: Cell<u32>, // 状態変数 その1
    y_cell: Cell<u32>, // 状態変数 その2
    z_cell: Cell<u32>, // 状態変数 その3
    w_cell: Cell<u32>, // 状態変数 その4
}

