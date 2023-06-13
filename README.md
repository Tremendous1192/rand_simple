# rand_simple
[![Crate](https://img.shields.io/crates/v/rand_simple.svg)](https://crates.io/crates/rand_simple)

このライブラリは、疑似乱数を簡単に生成できるライブラリです。

例えば、```use rand_simple::Uniform;```と宣言するだけで、一様分布乱数の構造体を使えるようになります。

偉大な先達[rand](https://crates.io/crates/rand)と比較して、簡素なモジュール宣言と豊富な確率変数による使いやすさを目指しています。

# 実装する疑似乱数
[計算機シミュレーションのための確率分布乱数生成法/著者　四辻 哲章/プレアデス出版](http://www.pleiades-publishing.co.jp/pdf/pdf03.html)に掲載されている確率分布乱数を実装していきます。

掲載されている46種類の確率分布と、その基礎である一様分布・ベルヌーイ分布を実装していきます。

基本的なアルゴリズムは[Xorshift160](https://www.jstatsoft.org/article/view/v008i14)です。

Xorshift160はたった5つの状態変数から周期 $2^{160} - 1$ の乱数計算を行うことができます。

1マイクロ秒毎に乱数を生成したとしても、ループが完了するまで$10^{34}$年かかるそうなので、
ちょっとした乱数生成の範疇ではこれで十分だと考えています。

# 外部ライブラリからの独立
使い勝手を考慮して、外部ライブラリに依存しないライブラリを目指しています。

# 使用例
## 一様分布
#### new()関数でインスタンスを生成する
```rust
let mut uniform = rand_simple::Uniform::new(1192u32);
println!("初期設定の場合、閉区間[0, 1]の一様乱数に従う乱数を返す -> {}", uniform.sample());

// 確率変数のパラメータを変更する場合
let min: f64 = -1f64;
let max: f64 = 1f64;
let result: Result<(f64, f64), &str> = uniform.try_set_params(min, max);
println!("閉区間[{}, {}]の一様乱数を生成する -> {}", min, max, uniform.sample());
```
#### インスタンス生成マクロ 1(他の構造体にもcreate_XXの生成マクロを用意しています)
```rust
let mut uniform = rand_simple::create_uniform!(1192u32);
println!("乱数 -> {}", uniform.sample());
```
#### インスタンス生成マクロ 2 (現在時刻を基にして乱数の種を設定する場合)
```rust
let mut uniform = rand_simple::create_uniform!();
println!("乱数: {}", uniform.sample()); // インスタンス生成時刻に依存するため、コンパイル時は値不明
```
