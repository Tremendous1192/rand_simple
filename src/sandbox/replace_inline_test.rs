use std::cell::Cell;
use std::time::{SystemTime};
use std::mem;

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
    // opt-level = 0 の場合、百万個生成したときの計算時間[milli sec]
    // (104, 97, 94, 103, 105, 36, 53)
    // シンプルな可変参照が早かった。
    assert_eq!((_span_educational, _span_closure, _span_closure_2,
        _span_inline, _span_inline2,
        _span_mut_educational, _span_mut_mem),
    (0, 0, 0, 0, 0, 0, 0));
}