// 基本計算
// 論文
// Marsaglia, G. (2003). Xorshift RNGs. Journal of Statistical Software, 8(14), 1?6. https://doi.org/10.18637/jss.v008.i14
// 参考コード
// http://www.6809.net/tenk/html/prog/xorshiftrand/XorShiftRand.h.html
pub fn xorshift160 (x: &mut u32, y: &mut u32, z: &mut u32, u: &mut u32, v: &mut u32) -> u32 {
    let t = *x ^ (*x << 7u32);
    *x = *y;
    *y = *z;
    *z = *u;
    *u = *v;
    *v = (*v ^ (*v >> 6u32)) ^ (t ^ (t >> 13u32));
    *v
}
// 一様乱数を計算するための分母
const MAX_U32_AS_F64: f64 = std::u32::MAX as f64;


// 標準正規分布の定数
//const A_NORMAL: f64 = 1.17741002252_f64; // √(ln4)
const B_NORMAL: f64 = 2.50662827463_f64; // √(2π)
const S_NORMAL: f64 = 0.88579134438_f64; // a / (b - a)
const K_NORMAL: u32 = 30783_u32; // floor( (2^(m/2) - 1) * a / b )
const W_NORMAL: f64 = 0.00003824869_f64; // b / (2^(m/2) - 1)
const P_NORMAL: f64 = 0.94289567219_f64; // (s + 1) / 2
const Q_NORMAL: f64 = -0.12127385907_f64; // ln(s)
const HALF_BIT_NORMAL: u32 = 65535_u32; // 2^(m/2) - 1
// 標準正規分布
// アルゴリズム 3.5: onty Python法
pub fn standard_normal (x0: &mut u32, y0: &mut u32, z0: &mut u32, u0: &mut u32, v0: &mut u32,
    x1: &mut u32, y1: &mut u32, z1: &mut u32, u1: &mut u32, v1: &mut u32) -> f64 {
    // step 1: m bit符号無整数型の一様乱数の生成
    let u_mbit_integer: u32 = xorshift160(x0, y0, z0, u0, v0);
    // step 2: 乱数の符号を最下位ビットで計算する
    let sign: f64 = if (u_mbit_integer & 1u32) == 1u32 {1f64} else {-1f64};
    // 1ビット右シフトしたものを準備する
    let u_m_1: u32 = u_mbit_integer >> 1u32;
    // step 3: (m/2) bitとの論理積を計算する
    let u_half_m_integer: u32 = u_m_1 & HALF_BIT_NORMAL;
    // step 4: u_x = u_half_m_integer * W;
    let u_x: f64 = u_half_m_integer as f64 * W_NORMAL;
    // step 5: u_half_m_integer < K の場合、y = sign * u_x を返す
    if u_half_m_integer < K_NORMAL {sign * u_x}
    else {
        // step 6: u_m_1 をさらに右に(m/2)ビットシフトする
        let u_half_m_1 = u_m_1 >> 16u32;
        // step 7: u_dash = (u_half_m_1 as f64 + 0.5f64) / (2^(m/2) - 2)
        let u_dash: f64 = (u_half_m_1 as f64 + 0.5f64) / 65534_f64;
        // step 8: ln(u_dash) < - u_x^2 / 2 のとき、y = sign * ux を返す
        if u_dash.ln() < - u_x.powi(2i32) / 2f64 { sign * u_x }
        else {
            // step 9: yを計算して、最後の分岐
            let y: f64 = sign * S_NORMAL * (B_NORMAL - u_x);
            if (P_NORMAL - u_dash).ln() < Q_NORMAL - y.powi(2i32) / 2f64 { y }
            else {
                // step 10: アルゴリズム 3.1*の裾野の計算
                sign * standard_normal_foot(x0, y0, z0, u0, v0, x1, y1, z1, u1, v1)
            }
        }
    }
}

const D_NORMAL: f64 = 6.28318530718_f64; // √(2π)
// アルゴリズム 3.13
fn standard_normal_foot (x0: &mut u32, y0: &mut u32, z0: &mut u32, u0: &mut u32, v0: &mut u32,
    x1: &mut u32, y1: &mut u32, z1: &mut u32, u1: &mut u32, v1: &mut u32) -> f64 {
    loop {
        // step 2: (0, 1) と [0, 1] の一様乱数を生成する
        let mut u_1: u32 = xorshift160(x0, y0, z0, u0, v0);
        let mut u_2: f64 = (xorshift160(x1, y1, z1, u1, v1) as f64) / MAX_U32_AS_F64;
        if u_1 != 0 && u_1 != std::u32::MAX {
            let x: f64 = (D_NORMAL - 2f64 * (1f64 - u_1 as f64 / MAX_U32_AS_F64).ln()).sqrt();
            if x * u_2 <= B_NORMAL {
                return x;
            }
        }
    }
}