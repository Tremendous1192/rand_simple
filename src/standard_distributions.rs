// 基本計算
// 論文
// Marsaglia, G. (2003). Xorshift RNGs. Journal of Statistical Software, 8(14), 1?6. https://doi.org/10.18637/jss.v008.i14
// 参考コード
// http://www.6809.net/tenk/?%e9%9b%91%e8%a8%98%2f2010
// http://www.6809.net/tenk/html/prog/xorshiftrand/XorShiftRand.h.html
#[inline]
pub(crate) fn xorshift160(xyzuv: &mut [u32; 5]) -> u32 {
    let t = xyzuv[0] ^ (xyzuv[0] << 7_u32);
    xyzuv[0] = xyzuv[1];
    xyzuv[1] = xyzuv[2];
    xyzuv[2] = xyzuv[3];
    xyzuv[3] = xyzuv[4];
    xyzuv[4] = (xyzuv[4] ^ (xyzuv[4] >> 6_u32)) ^ (t ^ (t >> 13_u32));
    xyzuv[4]
}

// 閉区間[0, 1]の一様乱数
#[inline]
pub(crate) fn xorshift160_0_1(xyzuv: &mut [u32; 5]) -> f64 {
    xorshift160(xyzuv) as f64 / MAX_U32_AS_F64
}

// 区間[0, 1)の一様乱数
#[inline]
pub(crate) fn xorshift160_0_1_open(xyzuv: &mut [u32; 5]) -> f64 {
    loop {
        xorshift160(xyzuv);
        if xyzuv[4] != std::u32::MAX {
            return xyzuv[4] as f64 / MAX_U32_AS_F64;
        }
    }
}

// 開区間(0, 1)の一様乱数
#[inline]
pub(crate) fn xorshift160_0_open_1_open(xyzuv: &mut [u32; 5]) -> f64 {
    loop {
        xorshift160(xyzuv);
        if xyzuv[4] != 0_u32 && xyzuv[4] != std::u32::MAX {
            return xyzuv[4] as f64 / MAX_U32_AS_F64;
        }
    }
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
/// 標準正規分布
/// アルゴリズム 3.5: Monty Python法
#[inline]
pub(crate) fn standard_normal(xyzuv0: &mut [u32; 5], xyzuv1: &mut [u32; 5]) -> f64 {
    // step 1: m bit符号無整数型の一様乱数の生成
    let u_mbit_integer: u32 = xorshift160(xyzuv0);
    // step 2: 乱数の符号を最下位ビットで計算する
    let sign: f64 = if (u_mbit_integer & 1_u32) == 1_u32 {
        1_f64
    } else {
        -1_f64
    };
    // 1ビット右シフトしたものを準備する
    let u_m_1: u32 = u_mbit_integer >> 1_u32;
    // step 3: (m/2) bitとの論理積を計算する
    let u_half_m_integer: u32 = u_m_1 & HALF_BIT_NORMAL;
    // step 4: u_x = u_half_m_integer * W;
    let u_x: f64 = u_half_m_integer as f64 * W_NORMAL;
    // step 5: u_half_m_integer < K の場合、y = sign * u_x を返す
    if u_half_m_integer < K_NORMAL {
        sign * u_x
    } else {
        // step 6: u_m_1 をさらに右に(m/2)ビットシフトする
        let u_half_m_1 = u_m_1 >> 16_u32;
        // step 7: u_dash = (u_half_m_1 as f64 + 0.5f64) / (2^(m/2) - 2)
        let u_dash: f64 = (u_half_m_1 as f64 + 0.5_f64) / 65534_f64;
        // step 8: ln(u_dash) < - u_x^2 / 2 のとき、y = sign * ux を返す
        if u_dash.ln() * 2_f64 < -u_x.powi(2_i32) {
            sign * u_x
        } else {
            // step 9: yを計算して、最後の分岐
            let y: f64 = sign * S_NORMAL * (B_NORMAL - u_x);
            if (P_NORMAL - u_dash).ln() < Q_NORMAL - y.powi(2_i32) / 2_f64 {
                y
            } else {
                // step 10: アルゴリズム 3.1*の裾野の計算
                sign * standard_normal_foot(xyzuv0, xyzuv1)
            }
        }
    }
}

const D_NORMAL: f64 = core::f64::consts::TAU; // b^2 = 2π
/// 標準正規分布の裾野
/// アルゴリズム 3.13
#[inline]
fn standard_normal_foot(xyzuv0: &mut [u32; 5], xyzuv1: &mut [u32; 5]) -> f64 {
    loop {
        // step 2: (0, 1) と [0, 1] の一様乱数を生成する
        let u_1: f64 = xorshift160_0_open_1_open(xyzuv0);
        let u_2: f64 = xorshift160_0_1(xyzuv1);
        // 乱数 x を計算する
        let x: f64 = (D_NORMAL - 2_f64 * (1_f64 - u_1).ln()).sqrt();
        // step 3: 条件分岐
        if x * u_2 <= B_NORMAL {
            return x;
        }
    }
}

/// 標準コーシー分布
/// アルゴリズム　3.26: (逆関数法)
#[inline]
pub(crate) fn standard_cauchy(xyzuv0: &mut [u32; 5]) -> f64 {
    // step 1: 開区間 (0, 1) の一様乱数
    let u = xorshift160_0_open_1_open(xyzuv0);
    // step 2: 標準分布を計算する
    (std::f64::consts::PI * (u - 0.5_f64)).tan()
}

/// 標準指数分布
/// アルゴリズム 3.41: 逆関数法
#[inline]
pub(crate) fn standard_exponential(xyzuv: &mut [u32; 5] /*, u_1: &mut f64*/) -> f64 {
    // step 1: [0, 1) の一様乱数を生成する
    // step 2: y = -ln(1 - u) を計算する
    -(1_f64 - xorshift160_0_1_open(xyzuv)).ln()
}

/// 標準ラプラス分布
/// アルゴリズム 3.45
#[inline]
pub(crate) fn standard_laplace(xyzuv: &mut [u32; 5] /* , u_1: &mut f64*/) -> f64 {
    // step 1: (0, 1) の一様乱数の生成
    let u: f64 = xorshift160_0_open_1_open(xyzuv);
    // step 2: 分岐
    if u < 0.5_f64 {
        (2_f64 * u).ln()
    } else {
        -(2_f64 * (1_f64 - u)).ln()
    }
}

// 標準ガンマ分布
// アルゴリズム 3.60
#[inline]
pub(crate) fn standard_gamma(
    xyzuv: &mut [u32; 5],
    //u_1: &mut f64,
    xyzuv0: &mut [u32; 5],
    xyzuv1: &mut [u32; 5],
    alpha: &f64,
) -> f64 {
    // α = 1 のとき標準指数分布を返す
    if *alpha == 1_f64 {
        return standard_exponential(xyzuv /*, u_1*/);
    }
    // α < 1 のときは回帰関数で計算する
    else if *alpha < 1_f64 {
        return standard_gamma(xyzuv, /* u_1,*/ xyzuv0, xyzuv1, &(alpha + 1_f64))
            * xorshift160_0_open_1_open(xyzuv).powf(1_f64 / *alpha);
    }
    // 前処理
    let d = *alpha - 1_f64 / 3_f64;
    let c = (9_f64 * d).powf(-0.5);
    loop {
        // step 1
        let z = standard_normal(xyzuv0, xyzuv1);
        let v = 1_f64 + c * z;
        // step 2
        if v > 0_f64 {
            let w = v.powi(3);
            let y = d * w;
            // step 3
            let u: f64 = xorshift160_0_open_1_open(xyzuv);
            if u <= 1_f64 - 0.0331 * z.powi(4) {
                // step 5
                return y;
            }
            // step 4
            if z.powi(2) / 2_f64 + d * (w.ln() + 1_f64) - y >= u.ln() {
                // step 5
                return y;
            }
        }
    }
}
