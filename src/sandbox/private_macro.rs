// 別のモジュールにあるマクロを呼び出す方法を調べた。

// 参考 : 回答にあるtrickを用いれば良いことがわかった
// rust - How do I use a macro across module files? - Stack Overflow
// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files

// 引用 : 下記の最小値を返すマクロで考える。
// 17.1.3. Repeat - Rust By Example
// https://doc.rust-lang.org/rust-by-example/macros/repeat.html
#[cfg(test)]
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}
pub(crate) use find_min; // このトリックをつけることで、他のモジュールでもマクロを使用できる。

// 同じモジュールにあるマクロをテストする
#[test]
fn test_same_mod() {
    assert_eq!(find_min!(1u64), 1u64);
    assert_eq!(find_min!(1u64 + 2u64, 2u64), 2u64);
    assert_eq!(find_min!(5u64, 2u64 * 3u64, 4u64), 4u64);
}

// 子モジュールにあるマクロをテストする
#[test]
fn test_child_mod() {
    assert_eq!(child::child_find_min!(1u64), 1u64);
    assert_eq!(child::child_find_min!(1u64 + 2u64, 2u64), 2u64);
    assert_eq!(child::child_find_min!(5u64, 2u64 * 3u64, 4u64), 4u64);
}

// 子モジュール
#[cfg(test)]
pub(super) mod child {
    // 子モジュール内のマクロ
    #[cfg(test)]
    macro_rules! child_find_min {
        // Base case:
        ($x:expr) => ($x);
        // `$x` followed by at least one `$y,`
        ($x:expr, $($y:expr),+) => (
            // Call `find_min!` on the tail `$y`
            std::cmp::min($x, find_min!($($y),+))
        )
    }
    pub(crate) use child_find_min; // このトリックをつけることで、他のモジュールでもマクロを使用できる。

    // 同じモジュールにあるマクロをテストする
    #[test]
    fn test_same_mod() {
        assert_eq!(child_find_min!(1u64), 1u64);
        assert_eq!(child_find_min!(1u64 + 2u64, 2u64), 2u64);
        assert_eq!(child_find_min!(5u64, 2u64 * 3u64, 4u64), 4u64);
    }

    // 親モジュールにあるマクロをテストする
    #[test]
    fn test_super_mod() {
        assert_eq!(super::find_min!(1u64), 1u64);
        assert_eq!(super::find_min!(1u64 + 2u64, 2u64), 2u64);
        assert_eq!(super::find_min!(5u64, 2u64 * 3u64, 4u64), 4u64);
    }
}


