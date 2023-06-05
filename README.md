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

# 外部ライブラリからの独立
使い勝手を考慮して、外部ライブラリに依存しないライブラリを目指しています。

# 使用例
## 一様分布
#### new()関数でインスタンスを生成する
```rust
let uniform = rand_simple::Uniform::new(1192u32);
// 初期設定の場合、閉区間[0, 1]の一様乱数に従う乱数を返す
assert_eq!(uniform.sample(), 0.8512317447111084f64);

// 確率変数のパラメータを変更する場合
let min: f64 = -1f64;
let max: f64 = 1f64;
let result: Result<(f64, f64), &str> = uniform.try_set_params(min, max);
assert_eq!(uniform.sample(), -0.7648924006533093f64);
```
#### インスタンス生成マクロ 1(他の構造体にもcreate_XXの生成マクロを用意しています)
```rust
let uniform = rand_simple::create_uniform!(1192u32);
assert_eq!(uniform.sample(), 0.8512317447111084f64);
```
#### インスタンス生成マクロ 2
```rust
let uniform = rand_simple::create_uniform!();
println!("乱数: {}", uniform.sample()); // インスタンス生成時刻に依存するため、コンパイル時は値不明
```
## 正規分布
```rust
let normal = rand_simple::Normal::new(1192u32, 765u32);
assert_eq!(normal.sample(), 0.11478775584530312f64); // 平均値 0, 標準偏差 1 の標準正規分布
```
## 半正規分布
```rust
let half_normal = rand_simple::HalfNormal::new(1192u32, 765u32);
assert_eq!(half_normal.sample(), 1.8943489630074781f64); // 標準偏差 1 の標準半正規分布
```
## コーシー分布
```rust
let cauchy = rand_simple::Cauchy::new(1192u32, 765u32);
assert_eq!(cauchy.sample(), 0.9999997103138784f64); // 位置母数 μ = 0, 尺度母数 θ = 1の乱数
```
## 半コーシー分布
```rust
let half_cauchy = rand_simple::HalfCauchy::new(1192u32, 765u32);
assert_eq!(half_cauchy.sample(), 0.9999971261133705f64); // 尺度母数 θ = 1の乱数
```
## レヴィ分布
```rust
let levy = rand_simple::Levy::new(1192u32, 765u32);
assert_eq!(levy.sample(), 0.27866346364478645f64); // 位置母数 μ = 0, 尺度母数 θ = 1の乱数
```
## 指数分布
```rust
let exponential = rand_simple::Exponential::new(1192u32);
assert_eq!(exponential.sample(), 1.5180935542424843f64); // 尺度母数 θ = 1の乱数
```
## ラプラス分布
```rust
let laplace = rand_simple::Laplace::new(1192u32);
assert_eq!(laplace.sample(), -0.824946373682539f64); // 位置母数 μ = 0, 尺度母数 θ = 1の乱数
```
## ベルヌーイ分布
```rust
let bernoulli = rand_simple::Bernoulli::new(1192u32);
assert_eq!(bernoulli.sample(), 0u64); // 発生確率 0.5の事象が生じたか(1u64)、否か(0u64)
```
## 幾何分布
```rust
let geometric = rand_simple::Geometric::new(1192u32);
assert_eq!(geometric.sample(), 2u64); // 発生確率 0.5の事象が初めて生じるまでの試行回数
```