# rand_simple
[![Crate](https://img.shields.io/crates/v/rand_simple.svg)](https://crates.io/crates/rand_simple)

このライブラリは、疑似乱数を簡単に呼び出すことができるライブラリです。

例えば、```use rand_simple::Uniform;```と宣言するだけで、一様分布乱数を使用できます。

偉大な先達[rand](https://crates.io/crates/rand)と比較して、簡素なモジュール宣言による差別化を図っています。

# 乱数の周期
使用しているアルゴリズムは[Xorshift](https://ja.wikipedia.org/wiki/Xorshift)です。

Xorshiftはたった4つの状態変数を記憶して周期 $2^{128} - 1$ の乱数計算を行うことができます。

0.001秒ごとに乱数を生成したとしても、ループが完了するまで$10^{28}$年かかるそうなので、
ちょっとした乱数生成の範疇ではこれで十分だと考えています。

# 使用例
## 一様分布
```rs
use rand_simple::Uniform;
let uniform = Uniform::new(1192u32);
let next = uniform.next_double();
println!("乱数: {}", next); // 0.8698977918526851f64
```
## ベルヌーイ分布
```rs
use rand_simple::Bernoulli;
let bernoulli = Bernoulli::new(1192u32);
let next = bernoulli.next_uint(0.5f64);
println!("乱数: {}", next); // 0u32
```
