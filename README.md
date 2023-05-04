# rand_simple
[![Crate](https://img.shields.io/crates/v/rand_simple.svg)](https://crates.io/crates/rand_simple)

このライブラリは、疑似乱数を簡単に生成できるライブラリです。

例えば、```use rand_simple::Uniform;```と宣言するだけで、一様分布乱数の構造体を使えるようになります。

偉大な先達[rand](https://crates.io/crates/rand)と比較して、簡素なモジュール宣言と豊富な確率変数による使いやすさを目指しています。

# 実装する疑似乱数
[計算機シミュレーションのための確率分布乱数生成法/著者　四辻 哲章/プレアデス出版](http://www.pleiades-publishing.co.jp/pdf/pdf03.html)に掲載されている確率分布乱数を実装していきます。

掲載されている46種類の確率分布と、その基礎である一様分布・ベルヌーイ分布を実装していきます。

基本的なアルゴリズムは[Xorshift](https://ja.wikipedia.org/wiki/Xorshift)です。

Xorshiftはたった4つの状態変数から周期 $2^{128} - 1$ の乱数計算を行うことができます。

0.001秒ごとに乱数を生成したとしても、ループが完了するまで$10^{28}$年かかるそうなので、
ちょっとした乱数生成の範疇ではこれで十分だと考えています。

# 使用例
## 一様分布
#### new関数
```rust
use rand_simple::Uniform;
let uniform = Uniform::new(1192u32);
let next = uniform.sample(); // 閉区間[0, 1]の一様乱数
println!("乱数: {}", next); // 0.8698977918526851f64
```
#### マクロ・引数有り(他の構造体にもcreate_XXの生成マクロを用意しています)
```rust
use rand_simple::create_uniform;
let uniform = create_uniform!(1192u32);
let next = uniform.sample(); // 閉区間[0, 1]の一様乱数
println!("乱数: {}", next); // 0.8698977918526851f64
```
#### マクロ・引数無し
```rust
use rand_simple::create_uniform;
let uniform = create_uniform!();
let next = uniform.sample(); // 閉区間[0, 1]の一様乱数
println!("乱数: {}", next); // 値不明
```
## 正規分布
```rust
use rand_simple::Normal;
let normal = Normal::new(1192u32, 765u32);
let next = normal.sample(); // 平均値 0, 標準偏差 1 の標準正規分布
println!("乱数: {}", next); // -1.2296205447119757
```
## 半正規分布
```rust
use rand_simple::HalfNormal;
let half_normal = HalfNormal::new(1192u32, 765u32);
let next = half_normal.sample(); // 標準偏差 1 の標準半正規分布
println!("乱数: {}", next); // 2.5308912695634582
```
## コーシー分布
```rust
use rand_simple::Cauchy;
let cauchy = Cauchy::new(1192u32, 765u32);
let next = cauchy.sample(); // 位置母数 μ = 0, 尺度母数 θ = 1の乱数
println!("乱数: {}", next); // 1.0046339315561652f64
```
## 半コーシー分布
```rust
use rand_simple::HalfCauchy;
let half_cauchy = HalfCauchy::new(1192u32, 765u32);
let next = half_cauchy.sample(); // 尺度母数 θ = 1の乱数
println!("乱数: {}", next); // 0.9999951805774843f64
```
## レヴィ分布
```rust
use rand_simple::Levy;
let levy = Levy::new(1192u32, 765u32);
let next = levy.sample(); // 位置母数 μ = 0, 尺度母数 θ = 1の乱数
println!("乱数: {}", next); // 0.15611801640551176f64
```
## 指数分布
```rust
use rand_simple::Exponential;
let let exponential = Exponential::new(1192u32);
let next = exponential.sample(); // 尺度母数 θ = 1の乱数
println!("乱数: {}", next); // 1.4145870106554208f64
```
## ベルヌーイ分布
```rust
use rand_simple::Bernoulli;
let bernoulli = Bernoulli::new(1192u32);
let next = bernoulli.sample(0.5f64); // 発生確率 0.5の事象が生じたか(1)、否か(0)
println!("乱数: {}", next); // 0u32
```
## 幾何分布
```rust
use rand_simple::Geometric;
let geometric = Geometric::new(1192u32);
let next = geometric.sample(0.5f64); // 発生確率 0.5の事象が初めて生じるまでの試行回数
println!("乱数: {}", next); // 4u32
```
# 外部ライブラリからの独立
使い勝手を考慮して、外部ライブラリに依存しないライブラリを目指しています。