# rand_simple
[![Crate](https://img.shields.io/crates/v/rand_simple.svg)](https://crates.io/crates/rand_simple)

このライブラリは、疑似乱数を簡単に生成できるライブラリです。

例えば、```use rand_simple::Uniform;```と宣言するだけで、一様分布乱数の構造体を使えるようになります。

偉大な先達[rand](https://crates.io/crates/rand)と比較して、簡素なモジュール宣言と豊富な確率変数による使いやすさを目指しています。

## 豊富な疑似乱数
[計算機シミュレーションのための確率分布乱数生成法/著者　四辻 哲章/プレアデス出版](http://www.pleiades-publishing.co.jp/pdf/pdf03.html)に掲載されている40種類以上の確率分布乱数と、その基礎である一様分布・ベルヌーイ分布を実装していきます。

基礎となるアルゴリズムは[Xorshift160](https://www.jstatsoft.org/article/view/v008i14)で、たった5つの状態変数から周期 $2^{160} - 1$ 個の乱数計算を行うことができます。

1マイクロ秒毎に乱数を生成したとしても、ループが完了するまで$10^{34}$年かかるそうなので、ちょっとした乱数生成の範疇ではこれで十分だと考えています。

## 使用例
### 一様分布
```rust
let seed:u32 = rand_simple::generate_seeds!(1_usize)[0];
let mut uniform = rand_simple::Uniform::new(seed);
println!("初期設定の場合、閉区間[0, 1]の一様乱数に従う乱数を返す -> {}", uniform.sample());

// 確率変数のパラメータを変更する場合
let min: f64 = -1_f64;
let max: f64 = 1_f64;
let result: Result<(f64, f64), &str> = uniform.try_set_params(min, max);
println!("閉区間[{}, {}]の一様乱数を生成する -> {}", min, max, uniform.sample());
```
### 正規分布
```rust
let seeds:[u32; 2_usize] = rand_simple::generate_seeds!(2_usize);
let mut normal = rand_simple::Normal::new(seeds);
println!("初期設定の場合、平均値 μ = 0, 分散 σ^2 = 1 の標準正規分布乱数を生成する -> {}", normal.sample());

// 確率変数のパラメータを変更する場合
let mean: f64 = -3_f64;
let variance: f64 = 2_f64;
let result: Result<(f64, f64), &str> = normal.try_set_params(mean, variance);
println!("平均値 μ = {}, 分散 σ^2 = {} の正規分布乱数を生成する -> {}", mean, variance, normal.sample());
```


## 実装状況
### 連続型
* [x] 一様分布
* [x] 3.1 正規分布
* [x] 3.2 半正規分布
* [x] 3.3 対数正規分布
* [x] 3.4 コーシー分布
  * [x] 半コーシー分布
* [x] 3.5 レヴィ分布
* [x] 3.6 指数分布
* [x] 3.7 ラプラス分布
  * [x] 対数ラプラス分布
* [x] 3.8 レイリー分布
* [x] 3.9 ワイブル分布
  * [x] 反射ワイブル分布
  * [x] フレシェ分布
* [x] 3.10 ガンベル分布
* [x] 3.11 ガンマ分布
* [x] 3.12 ベータ分布
* [ ] 3.13 ディリクレ分布
* [x] 3.14 べき関数分布
* [ ] 3.15 指数べき分布
* [x] 3.16 アーラン分布
* [x] 3.17 $\chi^2$ 分布
* [x] 3.18 $\chi$ 分布
* [x] 3.19 F分布
* [ ] 3.20 t分布
* [ ] 3.21 逆ガウス分布
* [ ] 3.22 三角分布
* [ ] 3.23 パレート分布
* [ ] 3.24 ロジスティック分布
* [ ] 3.25 双曲線正割分布
* [ ] 3.26 余弦分布
* [ ] 3.27 逆正弦分布
* [ ] 3.28 フォン・ミーゼス分布
* [ ] 3.29 非心ガンマ分布
* [ ] 3.30 非心ベータ分布
* [ ] 3.31 非心 $\chi^2$ 分布
* [ ] 3.32 非心 $\chi$ 分布
* [ ] 3.33 非心F分布
* [ ] 3.34 非心t分布
* [ ] 3.35 プランク分布
### 離散型
* [x] ベルヌーイ分布分布
* [ ] 4.1 二項分布
* [x] 4.2 幾何分布
* [ ] 4.3 ポアソン分布
* [ ] 4.4 超幾何分布
* [ ] 4.5 多項分布
* [ ] 4.6 負の二項分布
* [ ] 4.7 負の超幾何分布
* [ ] 4.8 対数級数分布
* [ ] 4.9 ユール・シモン分布
* [ ] 4.10 ジップ・マンデルブロート分布
* [ ] 4.11 ゼータ分布