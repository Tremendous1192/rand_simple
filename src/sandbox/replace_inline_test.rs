use std::cell::Cell;
use std::time::{SystemTime};
use std::mem;

// Xorshiftのおおもとの引用
// Marsaglia, G. (2003). Xorshift RNGs. Journal of Statistical Software, 8(14), 1?6. https://doi.org/10.18637/jss.v008.i14


// Xorshift 128 の分かりやすい実装
fn next_u32_educational (x: &Cell<u32>, y: &Cell<u32>, z: &Cell<u32>, w: &Cell<u32>) -> u32 {
    let t: u32 = x.get() ^ (x.get() << 11);
    let x_new: u32 = y.get();
    let y_new: u32 = z.get();
    let z_new: u32 = w.get();
    let w_new: u32 = (w.get() ^ (w.get() >> 19)) ^ (t ^ (t >>8));

    x.set(x_new);
    y.set(y_new);
    z.set(z_new);
    w.set(w_new);

    w.get()
}

// Xorshift 128 をクロージャーを用いて短く書き直した実装
fn next_u32_closure (x: &Cell<u32>, y: &Cell<u32>, z: &Cell<u32>, w: &Cell<u32>) -> u32 {
    // t = x ^ (x << 11), x_new = y, y_new = z, z_new = w
    let calculate_t = |arg: u32| arg ^ (arg << 11);
    let t: u32 = calculate_t(x.replace( y.replace( z.replace(w.get()) ) ));

    // w_ new = w ^ (w >> 19) ^ (t ^ (t >>8))
    let calculate_w = |arg: u32| (arg ^ (arg >> 19)) ^ (t ^ (t >> 8));
    w.set( calculate_w(w.take()) );
    w.get()
}

fn next_u32_closure_2 (x: &Cell<u32>, y: &Cell<u32>, z: &Cell<u32>, w: &Cell<u32>) -> u32 {
    // t = x ^ (x << 11), x_new = y, y_new = z, z_new = w
    let calculate_t = |arg: u32| arg ^ (arg << 11);
    //let t: u32 = calculate_t(x.replace( y.replace( z.replace(w.get()) ) ));

    // w_ new = w ^ (w >> 19) ^ (t ^ (t >>8))
    let calculate_w = |arg_t: u32, arg_w: u32| (arg_w ^ (arg_w >> 19)) ^ (arg_t ^ (arg_t >> 8));
    w.set( calculate_w(calculate_t(x.replace( y.replace( z.replace(w.get()) ) )), w.take()) );
    w.get()
}

#[inline]
fn next_t (x: u32) -> u32 {
    x ^ (x << 11)
}

#[inline]
fn next_w (t: u32, w: u32) -> u32 {
    (w ^ (w >> 19)) ^ (t ^ (t >> 8))
}

// Xorshift 128 をinline関数を用いて実装した。オーバーヘッドというのが減ると嬉しい
fn next_u32_inline (x: &Cell<u32>, y: &Cell<u32>, z: &Cell<u32>, w: &Cell<u32>) -> u32 {
    w.set(next_w(next_t(x.replace(y.replace(z.replace(w.get())))), w.take()));
    w.get()
}

// Xorshift 128 をinline関数を用いて実装した。オーバーヘッドというのが減ると嬉しい
fn next_u32_inline_2 (x: &Cell<u32>, y: &Cell<u32>, z: &Cell<u32>, w: &Cell<u32>) -> u32 {
    let t = next_t(x.replace(y.replace(z.replace(w.get()))));
    w.set(next_w(t, w.take()));
    w.get()
}

// 可変参照
fn next_u32_mut_educational (x: &mut u32, y: &mut u32, z: &mut u32, w: &mut u32) -> u32 {
    let t: u32 = *x ^ (*x << 11);
    *x = *y;
    *y = *z;
    *z = *w;
    *w = (*w ^ (*w >> 19)) ^ (t ^ (t >> 8));
    *w
}

// 可変参照 + mem
fn next_u32_mut_mem (x: &mut u32, y: &mut u32, z: &mut u32, w: &mut u32) -> u32 {
    let t: u32 = mem::replace(x, mem::replace(y, mem::replace(z, *w)));
    *w = (*w ^ (*w >> 19)) ^ (t ^ (t >> 8));
    *w
}


// オーバーフローを解決できなかった
/*
fn rotl (x: u32, k: u32) -> u32 {
    (x << k) | (x >> (32u32 - k))
}

// Xorshiro 128**??
// https://prng.di.unimi.it/xoshiro128starstar.c
// それとも Xorshiro 256**
// https://qiita.com/umireon/items/8c1f20c03a4d7b6b7587
fn next_u32_xorshiro_double_asterisk (x: &mut u32, y: &mut u32, z: &mut u32, w: &mut u32) -> u32 {
    let result: u32 = rotl(*y * 5u32, 7u32) * 9;

    let t: u32 = *y << 9;
    *z ^= *x;
    *w ^= *y;
    *y ^= *z;
    *x ^= *w;

    *z ^= t;

    *w = rotl(*w, 11);

    result
}
*/

// 64ビットの乱数
// https://prng.di.unimi.it/xorshift128plus.c
/*
fn next_u64 (x: &mut u64, y: &mut u64) -> u64 {
    let mut s1: u64 = *x;
    let s0: u64 = *y;
    let result: u64 = s0 + s1;
    *x = s0;
    s1 =s1 ^ (s1 << 23); // a
    *y = s1 ^ s0 ^ (s1 >> 18) ^ (s0 >> 5); // b, c
    result
}
*/


// Xorshiro 128++
// https://prng.di.unimi.it/xoshiro128plusplus.c
// オーバーフローするのでwrappingをかませたら、遅くなった。
// release版だと問題ないように思えるが、ちょっと微妙
fn rotl (x: u32, k: u32) -> u32 {
    //(x << k) | x >> (32 - k)
    x.wrapping_shl(k) | x.wrapping_shr(32u32 - k)
}
fn next_u32_xorshiro128plusplus (x: &mut u32, y: &mut u32, z: &mut u32, w: &mut u32) -> u32 {
    let result: u32 = rotl((*x).wrapping_add(*w), 7u32).wrapping_add(*x);

    let t: u32 = (*y).wrapping_shl(9u32); // *y << 9u32
    *z ^= *x;
    *w ^= *y;
    *y ^= *z;
    *x ^= *w;

    *z ^= t;

    *w = rotl(*w, 11u32);

    result
}




#[test]
fn check_same_results() {
    // 教育的な関数の種
    let x: Cell<u32> = Cell::new(123456789u32);
    let y: Cell<u32> = Cell::new(362436069u32);
    let z: Cell<u32> = Cell::new(521288629u32);
    let w: Cell<u32> = Cell::new(88675123u32);

    // クロージャーの種
    let x0: Cell<u32> = Cell::new(123456789u32);
    let y0: Cell<u32> = Cell::new(362436069u32);
    let z0: Cell<u32> = Cell::new(521288629u32);
    let w0: Cell<u32> = Cell::new(88675123u32);

    for _i in 0..100 {
        assert_eq!(next_u32_educational(&x, &y, &z, &w), next_u32_closure(&x0, &y0, &z0, &w0));
    }

    // クロージャーの種
    let x0: Cell<u32> = Cell::new(123456789u32);
    let y0: Cell<u32> = Cell::new(362436069u32);
    let z0: Cell<u32> = Cell::new(521288629u32);
    let w0: Cell<u32> = Cell::new(88675123u32);

    // インライン展開の種
    let x1: Cell<u32> = Cell::new(123456789u32);
    let y1: Cell<u32> = Cell::new(362436069u32);
    let z1: Cell<u32> = Cell::new(521288629u32);
    let w1: Cell<u32> = Cell::new(88675123u32);

    for _i in 0..100 {
        assert_eq!(next_u32_closure(&x0, &y0, &z0, &w0), next_u32_inline(&x1, &y1, &z1, &w1));
    }
}

#[test]
fn compare_time() {
    // 上書きする変数
    let mut _next: u32 = 0u32;
    // opt-level = 0 の場合、1000_000_0 個で約5秒かかる
    // opt-level = 2と3 の場合、u128の最大値まで繰り返しても差が分からない
    const REPEAT: u128 = 10;

    // 計測
    // 教育的
    let mut _span_educational: u128 = 0;
    {
        let x: Cell<u32> = Cell::new(123456789u32);
        let y: Cell<u32> = Cell::new(362436069u32);
        let z: Cell<u32> = Cell::new(521288629u32);
        let w: Cell<u32> = Cell::new(88675123u32);
        let initial = SystemTime::now();
        for _i in 0..REPEAT {
            _next = next_u32_educational(&x, &y, &z, &w);
        }
        let finish = SystemTime::now();
        _span_educational = finish.duration_since(initial).expect("Time went backwards").as_millis();
    }
    
    // クロージャー
    let mut _span_closure: u128 = 0;
    {
        let x: Cell<u32> = Cell::new(123456789u32);
        let y: Cell<u32> = Cell::new(362436069u32);
        let z: Cell<u32> = Cell::new(521288629u32);
        let w: Cell<u32> = Cell::new(88675123u32);
        let initial = SystemTime::now();
        for _i in 0..REPEAT {
            _next = next_u32_closure(&x, &y, &z, &w);
        }
        let finish = SystemTime::now();
        _span_closure = finish.duration_since(initial).expect("Time went backwards").as_millis();
    }
    
    
    // クロージャー その2
    let mut _span_closure_2: u128 = 0;
    {
        let x: Cell<u32> = Cell::new(123456789u32);
        let y: Cell<u32> = Cell::new(362436069u32);
        let z: Cell<u32> = Cell::new(521288629u32);
        let w: Cell<u32> = Cell::new(88675123u32);
        let initial = SystemTime::now();
        for _i in 0..REPEAT {
            _next = next_u32_closure_2(&x, &y, &z, &w);
        }
        let finish = SystemTime::now();
        _span_closure_2 = finish.duration_since(initial).expect("Time went backwards").as_millis();
    }
    
    // インライン展開
    let mut _span_inline: u128 = 0;
    {
        let x: Cell<u32> = Cell::new(123456789u32);
        let y: Cell<u32> = Cell::new(362436069u32);
        let z: Cell<u32> = Cell::new(521288629u32);
        let w: Cell<u32> = Cell::new(88675123u32);
        let initial = SystemTime::now();
        for _i in 0..REPEAT {
            _next = next_u32_inline(&x, &y, &z, &w);
        }
        let finish = SystemTime::now();
        _span_inline = finish.duration_since(initial).expect("Time went backwards").as_millis();
    }

    // インライン展開
    let mut _span_inline2: u128 = 0;
    {
        let x: Cell<u32> = Cell::new(123456789u32);
        let y: Cell<u32> = Cell::new(362436069u32);
        let z: Cell<u32> = Cell::new(521288629u32);
        let w: Cell<u32> = Cell::new(88675123u32);
        let initial = SystemTime::now();
        for _i in 0..REPEAT {
            _next = next_u32_inline_2(&x, &y, &z, &w);
        }
        let finish = SystemTime::now();
        _span_inline2 = finish.duration_since(initial).expect("Time went backwards").as_millis();
    }

    // 可変参照
    let mut _span_mut_educational: u128 = 0;
    {
        let mut x: u32 = 123456789u32;
        let mut y: u32 = 362436069u32;
        let mut z: u32 = 521288629u32;
        let mut w: u32 = 88675123u32;
        let initial = SystemTime::now();
        for _i in 0..REPEAT {
            _next = next_u32_mut_educational(&mut x, &mut y, &mut z, &mut w);
        }
        let finish = SystemTime::now();
        _span_mut_educational = finish.duration_since(initial).expect("Time went backwards").as_millis();
    }

    // 可変参照 + mem
    let mut _span_mut_mem: u128 = 0;
    {
        let mut x: u32 = 123456789u32;
        let mut y: u32 = 362436069u32;
        let mut z: u32 = 521288629u32;
        let mut w: u32 = 88675123u32;
        let initial = SystemTime::now();
        for _i in 0..REPEAT {
            _next = next_u32_mut_mem(&mut x, &mut y, &mut z, &mut w);
        }
        let finish = SystemTime::now();
        _span_mut_mem = finish.duration_since(initial).expect("Time went backwards").as_millis();
    }

    // 可変参照 6 Xoroshiro
    // オーバーフローしてしまう
    /*
    let mut _span_mut_xorshiro: u128 = 0;
    {
        let mut x: u32 = 123u32;
        let mut y: u32 = 362u32;
        let mut z: u32 = 521u32;
        let mut w: u32 = 886u32;
        let initial = SystemTime::now();
        for _i in 0..REPEAT {
            _next = next_u32_xorshiro_double_asterisk(&mut x, &mut y, &mut z, &mut w);
        }
        let finish = SystemTime::now();
        _span_mut_xorshiro = finish.duration_since(initial).expect("Time went backwards").as_millis();
    }
    */
    
    // xorshift128++
    // オーバーフローする
    /*
    let mut _span_mut_xorshft128pp: u128 = 0;
    let mut _next64: u64 = 1192u64;
    {
        let mut x: u64 = 1u64;
        let mut y: u64 = 6u64;
        let initial = SystemTime::now();
        for _i in 0..REPEAT {
            _next64 = next_u64(&mut x, &mut y);
        }
        let finish = SystemTime::now();
        _span_mut_xorshft128pp = finish.duration_since(initial).expect("Time went backwards").as_millis();
    }
    */

    // オーバーフローする

    let mut _span_mut_xorshiro128_pp: u128 = 0;
    {
        let mut x: u32 = 123456789u32;
        let mut y: u32 = 362436069u32;
        let mut z: u32 = 521288629u32;
        let mut w: u32 = 88675123u32;
        let initial = SystemTime::now();
        for _i in 0..REPEAT {
            _next = next_u32_xorshiro128plusplus(&mut x, &mut y, &mut z, &mut w);
        }
        let finish = SystemTime::now();
        _span_mut_xorshiro128_pp = finish.duration_since(initial).expect("Time went backwards").as_millis();
    }

    
    
    // opt-level = 0 の場合、百万個生成したときの計算時間[milli sec]
    // (52, 48, 48, 54, 51, 17, 26, 67)
    // シンプルな可変参照が早かった。
    assert_eq!((_span_educational, _span_closure, _span_closure_2,
        _span_inline, _span_inline2,
        _span_mut_educational, _span_mut_mem,
        _span_mut_xorshiro128_pp),
    (0, 0, 0, 0, 0, 0, 0, 0));
}