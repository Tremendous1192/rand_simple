// 基本計算
// 論文
// Marsaglia, G. (2003). Xorshift RNGs. Journal of Statistical Software, 8(14), 1?6. https://doi.org/10.18637/jss.v008.i14
// 参考コード
// http://www.6809.net/tenk/?%e9%9b%91%e8%a8%98%2f2010
// http://www.6809.net/tenk/html/prog/xorshiftrand/XorShiftRand.h.html
#[inline]
pub(crate) fn xorshift160 (xyzuv: &mut [u32; 5]) -> u32 {
    let t = xyzuv[0] ^ (xyzuv[0] << 7u32);
    xyzuv[0] = xyzuv[1];
    xyzuv[1] = xyzuv[2];
    xyzuv[2] = xyzuv[3];
    xyzuv[3] = xyzuv[4];
    xyzuv[4] = (xyzuv[4] ^ (xyzuv[4] >> 6u32)) ^ (t ^ (t >> 13u32));
    xyzuv[4]
}

// 閉区間[0, 1]の一様乱数
#[inline]
pub(crate) fn xorshift160_0_1 (xyzuv: &mut [u32; 5]) -> f64 {
    xorshift160(xyzuv) as f64 / MAX_U32_AS_F64
}

// 区間[0, 1)の一様乱数
#[inline]
pub(crate) fn xorshift160_0_1_open (xyzuv: &mut [u32; 5]) -> f64 {
    loop {
        xorshift160(xyzuv);
        if xyzuv[4] != std::u32::MAX {
            return xyzuv[4] as f64 / MAX_U32_AS_F64;
        }
    }
}

// 開区間(0, 1)の一様乱数
#[inline]
pub(crate) fn xorshift160_0_open_1_open (xyzuv: &mut [u32; 5]) -> f64 {
    loop {
        xorshift160(xyzuv);
        if xyzuv[4] != 0u32 && xyzuv[4] != std::u32::MAX {
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
// 標準正規分布
// アルゴリズム 3.5: Monty Python法
#[inline]
pub(crate) fn standard_normal (xyzuv0: &mut [u32; 5], xyzuv1: &mut [u32; 5]) -> f64 {
    // step 1: m bit符号無整数型の一様乱数の生成
    let u_mbit_integer: u32 = xorshift160(xyzuv0);
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
                sign * standard_normal_foot(xyzuv0, xyzuv1)
            }
        }
    }
}


const D_NORMAL: f64 = std::f64::consts::TAU; // b^2 = 2π
// 標準正規分布の裾野
// アルゴリズム 3.13
#[inline]
fn standard_normal_foot (xyzuv0: &mut [u32; 5], xyzuv1: &mut [u32; 5]) -> f64 {
    loop {
        // step 2: (0, 1) と [0, 1] の一様乱数を生成する
        let u_1: u32 = xorshift160(xyzuv0);
        let u_2: f64 = xorshift160_0_1(xyzuv1);
        if u_1 != 0 && u_1 != std::u32::MAX {
            // step 3: 条件分岐
            let x: f64 = (D_NORMAL - 2f64 * (1f64 - u_1 as f64 / MAX_U32_AS_F64).ln()).sqrt();
            if x * u_2 <= B_NORMAL {
                return x;
            }
        }
    }
}


// コーシー分布の定数
const B_CAUCHY: f64 = 4.766_f64;
const D_CAUCHY: f64 = 0.00000320169_f64; // 1/ (b(2^(m/2) - 2))
//const A_CAUCHY: f64 = 1.42622923652_f64; // √(2b/π - 1)
const K_CAUCHY: u32 = 30783_u32; // floor( (2^(m/2) - 1) * a / b )
const W_CAUCHY: f64 = 0.00007272449_f64; // b / (2^(m/2) - 1)
const S_CAUCHY: f64 = 0.42704405108_f64; // a / (b - a)
const P_CAUCHY: f64 = 2.34167879747_f64; // 1 / s
const Q_CAUCHY: f64 = 0.35057477942_f64; // (1 + p) / (2b)
const T_CAUCHY: f64 = 1.36397696_f64; // arctan(b)
const V_CAUCHY: f64 = 0.20681936679_f64; // arctan(b)
const HALF_BIT_CAUCHY: u32 = 65535_u32; // 2^(m/2) - 1
// 標準正規分布
// アルゴリズム 3.30: Monty Python法
#[inline]
pub(crate) fn standard_cauchy (xyzuv0: &mut [u32; 5], xyzuv1: &mut [u32; 5]) -> f64 {
    // step 1: m bit符号無整数型の一様乱数の生成
    let u_mbit_integer: u32 = xorshift160(xyzuv0);
    // step 2: 乱数の符号を最下位ビットで計算する
    let sign: f64 = if (u_mbit_integer & 1u32) == 1u32 {1f64} else {-1f64};
    // 1ビット右シフトしたものを準備する
    let u_m_1: u32 = u_mbit_integer >> 1u32;
    // step 3: (m/2) bitとの論理積を計算する
    let u_half_m_integer: u32 = u_m_1 & HALF_BIT_CAUCHY;
    // step 4: u_x = u_half_m_integer * W;
    let u_x: f64 = u_half_m_integer as f64 * W_CAUCHY;
    // step 5: u_half_m_integer < K の場合、y = sign * u_x を返す
    if u_half_m_integer < K_CAUCHY {sign * u_x}
    else {
        // step 6: u_m_1 をさらに右に(m/2)ビットシフトする
        let u_half_m_1 = u_m_1 >> 16u32;
        // step 7: u_y
        let u_y: f64 = D_CAUCHY * u_half_m_1 as f64;
        // step 8: π(1 + u_x^2)u_y < 1 のとき、y = sign * u_x を返す
        if std::f64::consts::PI * (1f64 + u_x.powi(2)) * u_y < 1f64 {sign * u_x}
        else {
            // step 9: yの計算と分岐
            let y: f64 = sign * S_CAUCHY * (B_CAUCHY - u_x);
            if std::f64::consts::PI * (1f64 + y.powi(2)) * (Q_CAUCHY - P_CAUCHY * u_y) < 1f64 {y}
            else {
                // step 10: 裾野?
                sign * (T_CAUCHY + V_CAUCHY * xorshift160_0_open_1_open(xyzuv1))
            }
        }
    }
}


// 指数関数の定数
const D_EXPONENTIAL: f64 = std::f64::consts::LN_2; // ln2
// 標準指数分布
// アルゴリズム 3.42
#[inline]
pub(crate) fn standard_exponential (xyzuv: &mut [u32; 5], u_1:&mut f64) -> f64 {
    // step 1: 前回生成した区間[0, 1)の一様乱数uを基に、次の一様乱数u'を生成する
    let u_dash: f64 = 1f64 - *u_1;
    // step 2: 重み a の初期化
    let mut a: f64 = 0f64;
    // step 3: u'' = 2u'
    // step 4: u'' < 1 のとき a の値をd増やし、u' = u''としてstep 3に戻る
    // u'' > 1のとき、u = u'' - 1とする
    let mut u_dash_dash: f64 = 2f64  * u_dash;
    while u_dash_dash < 1f64 {
        a += D_EXPONENTIAL;
        u_dash_dash *= 2f64;
    }
    *u_1 = u_dash_dash - 1f64;
    // step 5: w = d u_1 及び y = a + w 計算して、 k = 1 と設定する
    let mut w: f64 = D_EXPONENTIAL * *u_1;
    let mut k: u128 = 1u128;
    loop {
        // step 6: 区間[0, 1)の一様乱数 u_2 を生成する
        let u_2 = xorshift160_0_1_open (xyzuv);
        // step 7: u_2 < wのときは、w = u_2としてkの値を1増やして、step 6に戻る
        // u_2 ≧ wのときは、u_1 = (u_2 - w)/(1 - w)を計算する
        if u_2 < w {
            w = u_2;
            k += 1u128;
            continue;
        }
        else {
            *u_1 = (u_2 - 1f64) / (1f64 - w);
        }
        // step 8: kが偶数の時はstep 5に戻る
        if k % 2u128 == 0u128 {
            w = D_EXPONENTIAL * *u_1;
            k = 1u128;
        }
        else {
            break;
        }
    }
    // step 9: 所望の乱数を返す(y = a + w)
    a + w
}


// ラプラス分布の定数
const D_LAPLACE: f64 = std::f64::consts::LN_2; // ln2
// 標準ラプラス分布
// アルゴリズム 3.42
#[inline]
pub(crate) fn standard_laplace (xyzuv: &mut [u32; 5], u_1:&mut f64) -> f64 {
    // step 1: 前回生成した区間[0, 1)の一様乱数uを基に、次の一様乱数u'を生成する
    let u_dash: f64 = 1f64 - *u_1;
    // step 2: 符号の決定
    let sign: f64 = if u_dash < 1f64 {1f64} else {-1f64};
    let u_dash_dash: f64 = if u_dash < 1f64 {1f64 - u_dash} else {u_dash - 1f64};
    // step 3: 重み a の初期化
    let mut a: f64 = 0f64;
    // step 4: u'' = 2u'
    // step 5: u'' < 1 のとき a の値をd増やし、u' = u''としてstep 3に戻る
    // u'' > 1のとき、u = u'' - 1とする
    let mut u_dash_dash_dash: f64 = 2f64  * u_dash_dash;
    while u_dash_dash_dash < 1f64 {
        a += D_LAPLACE;
        u_dash_dash_dash *= 2f64;
    }
    *u_1 = u_dash_dash_dash - 1f64;
    // step 6: w = d u_1 及び y = a + w 計算して、 k = 1 と設定する
    let mut w: f64 = D_LAPLACE * *u_1;
    let mut k: u128 = 1u128;
    loop {
        // step 7: 区間[0, 1)の一様乱数 u_2 を生成する
        let u_2 = xorshift160_0_1_open (xyzuv);
        // step 8: u_2 < wのときは、w = u_2としてkの値を1増やして、step 7に戻る
        // u_2 ≧ wのときは、u_1 = (u_2 - w)/(1 - w)を計算する
        if u_2 < w {
            w = u_2;
            k += 1u128;
            continue;
        }
        else {
            *u_1 = (u_2 - 1f64) / (1f64 - w);
        }
        // step 9: kが偶数の時はstep 6に戻る
        if k % 2u128 == 0u128 {
            w = D_LAPLACE * *u_1;
            k = 1u128;
        }
        else {
            break;
        }
    }
    // step 10: 所望の乱数を返す(y = sign * (a + w))
    sign * (a + w)
}


// 標準ガンマ分布
// アルゴリズム 3.60
#[inline]
pub(crate) fn standard_gamma (xyzuv: &mut [u32; 5], u_1:&mut f64, xyzuv0: &mut [u32; 5], xyzuv1: &mut [u32; 5], alpha: &f64) -> f64 {
    // α = 1 のとき標準指数分布を返す
    if *alpha == 1f64 {
        return standard_exponential(xyzuv, u_1);
    }
    // α < 1 のときは回帰関数で計算する
    else if *alpha < 1f64 {
        return standard_gamma (xyzuv, u_1, xyzuv0, xyzuv1, &(alpha + 1f64)) * xorshift160_0_open_1_open(xyzuv).powf(1f64 / *alpha);
    }
    // 前処理
    let d = *alpha - 1f64 / 3f64;
    let c = (9f64 * d).powf(-0.5);
    loop {
        // step 1
        let z = standard_normal(xyzuv0, xyzuv1);
        let v = 1f64 + c * z;
        // step 2
        if v > 0f64 {
            let w = v.powi(3);
            let y = d * w;
            // step 3
            let u: f64 = xorshift160_0_open_1_open(xyzuv);
            if u <= 1f64 - 0.0331 * z.powi(4) {
                // step 5
                return y;
            }
            // step 4
            if z.powi(2) / 2f64 + d *(w.ln() + 1f64) - y >= u.ln() {
                // step 5
                return y;
            }
        }
    }
}
