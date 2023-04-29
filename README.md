# rand_simple
[![Crate](https://img.shields.io/crates/v/rand_simple.svg)](https://crates.io/crates/rand_simple)

このライブラリは、疑似乱数を簡単に呼び出すことができるライブラリです。

例えば、```use rand_simple::Uniform;```と宣言するだけで、一様分布乱数を使用できます。

偉大な先達[rand](https://crates.io/crates/rand)と比較して、簡素なモジュール宣言と豊富な確率変数による使いやすさを目指しています。

# 実装する疑似乱数
[計算機シミュレーションのため確率分布乱数生成法/著者　四辻 哲章/プレアデス出版](http://www.pleiades-publishing.co.jp/pdf/pdf03.html)に掲載されている確率分布乱数を実装していきます。

掲載されている46種類の確率分布と、その基礎である一様分布・ベルヌーイ分布を実装していきます。

基本的なアルゴリズムは[Xorshift](https://ja.wikipedia.org/wiki/Xorshift)です。

Xorshiftはたった4つの状態変数から周期 $2^{128} - 1$ の乱数計算を行うことができます。

0.001秒ごとに乱数を生成したとしても、ループが完了するまで$10^{28}$年かかるそうなので、
ちょっとした乱数生成の範疇ではこれで十分だと考えています。

# 使用例
## 一様分布
```rs
use rand_simple::Uniform;
let uniform = Uniform::new(1192u32);
let next = uniform.sample(); // 閉区間[0, 1]の一様乱数
println!("乱数: {}", next); // 0.8698977918526851f64
```
## 正規分布
```rs
use rand_simple::Normal;
let normal = Normal::new(1192u32, 765u32);
let next = normal.sample(); // 平均値 0, 標準偏差 1 の標準正規分布
println!("乱数: {}", next); // -1.2296205447119757
```
## ベルヌーイ分布
```rs
use rand_simple::Bernoulli;
let bernoulli = Bernoulli::new(1192u32);
let next = bernoulli.sample(0.5f64); // 発生確率 0.5の事象が生じたか(1)、否か(0)
println!("乱数: {}", next); // 0u32
```
## 幾何分布
```rs
use rand_simple::Geometric;
let geometric = Geometric::new(1192u32);
let next = geometric.sample(0.5f64); // 発生確率 0.5の事象が初めて生じるまでの試行回数
println!("乱数: {}", next); // 4u32
```
# 外部ライブラリからの独立
使い勝手を考慮して、外部ライブラリに依存しないライブラリを目指しています。

そして、デフォルトの時刻取得ライブラリが無いとのことで、よくある時刻による乱数の初期化はありません。