use crate::{Uniform, set_state, update_and_uniform};

impl Uniform {
    /// コンストラクタ
    /// * `_seed` - 乱数の種
    pub fn new(_seed: u32) -> Self {
        Self {
            xyzw: set_state(_seed),
        }
    }

    /// 閉区間[0, 1]の乱数を返す
    pub fn sample(&self) -> f64 {
        update_and_uniform(&self.xyzw)
    }
}


#[macro_export]
/// 一様分布のインスタンスを生成するマクロ
/// * `() =>` - 乱数の種は自動生成
/// * `($seed: expr) =>` - 乱数の種を指定する
macro_rules! create_uniform {
    () => {{
        $crate::Uniform::new($crate::create_seed())
    }};
    ($seed: expr) => {
        $crate::Uniform::new($seed as u32)
    };
}

// 以下、オーバーロードのテスト部分

use crate::TestUniformSample;
impl TestUniformSample for (f64, f64) {
    fn test_sample(uniform: &Uniform, foo: &Self) -> f64{
        let origin = foo.0.min(foo.1);
        let range = (foo.0 - foo.1).abs();
        update_and_uniform(&uniform.xyzw) * range + origin
    }
}
impl TestUniformSample for () {
    fn test_sample(uniform: &Uniform, _foo: &Self) -> f64{
        update_and_uniform(&uniform.xyzw)
    }
}

impl Uniform {
    /// (試験運用)一様分布乱数を返すメソッド
    /// #### オーバーロードテスト用
    /// # 使用例
    /// ```
    /// use rand_simple::Uniform;
    /// let uniform = Uniform::new(1192u32);
    /// // ↓確率変数のパラメータ無しを &() (unit tuple) で表現している
    /// let next_default = uniform.test_sample(&());
    /// println!("{}", next_default); // 引数が &() (unit tuple)の場合、閉区間[0, 1]で計算する
    /// let min: f64 = 0f64;
    /// let max: f64 = 2f64;
    /// let next_custom = uniform.test_sample(&(min, max));
    /// println!("{}", next_custom); // 引数が&(min, max)の場合、閉区間[min, max]で計算する
    /// ```
    /// traitによるオーバーロードのテストメソッド
    pub fn test_sample<T:TestUniformSample>(&self, foo: &T) -> f64 {
        T::test_sample(self, foo)
    }
}
#[test]
fn test_sample() {
    use crate::Uniform;

    // 最初に実装した乱数計算メソッド
    let uniform = Uniform::new(1192u32);
    let next_1: f64 = uniform.sample(); // 0.8512317447111084
    assert_eq!(next_1, 0.8512317447111084f64);

    // オーバーロード(引数 2個)
    let uniform_2 = Uniform::new(1192u32);
    let next_2 = uniform_2.test_sample(&(1f64, 0f64));
    assert_eq!(next_1, next_2);

    // オーバーロード(引数 0個)
    let uniform_3 = Uniform::new(1192u32);
    let next_3 = uniform_3.test_sample(&());
    assert_eq!(next_1, next_3);


    let uniform_4 = Uniform::new(1192u32);
    let next_4 = uniform_4.test_sample(&());
    assert_eq!(next_1, next_4);
}

// ここまで、オーバーロードのテスト部分


#[macro_export]
/// 確率変数を計算するマクロのテスト。変数名は固定
macro_rules! test_call_method {
    ($object: expr, $min: ident, $max: ident) => {{
        let range: f64 = ($max as f64 - $min as f64).abs();
        let origin: f64 = ($max as f64).min($min as f64);
        $object.sample() * range + origin
    }};
    // 正規分布等用
    ($object: expr, $mu: ident, $theta: ident) => {
        $object.sample() * ($theta as f64) + ($mu as f64)
    };
    ($object:expr) => {
        $object.sample()
    };
}

#[test]
fn test_macro_sample() {
    use crate::{Uniform, test_call_method};
    let uniform = Uniform::new(1192u32);
    let min: f64 = 0f64;
    let max: f64 = 1f64;
    assert_eq!(test_call_method!(uniform), 0.8512317447111084f64);
    assert_eq!(test_call_method!(uniform, min, max), 0.11755379967334535f64);
}

