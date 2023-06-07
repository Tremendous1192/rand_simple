// 2023年6月7日
// 0.0.36c (試行錯誤) 別モジュールのマクロを使用する方法の確認
// stack over flow様様である
#[cfg(test)] mod private_macro;

// 0.0.31 (破壊的変更) 確率分布のパラメータをフィールドに追加
// 2023年6月4日
// インスタンスにmutを宣言しなくとも、フィールドを変更する方法を模索した
// ビルダーパターンで実現できるとのことだが、所有権の移動に阻まれて余分なシャドウイングが必要だった。
// 参考 https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
#[cfg(test)] mod mutable_member_builder;
// 2023年6月4日(続き)
// 素直にCellを用いるのが一番分かりやすいが、乱数生成の度にパラメータをコピーするのか...?
// と思ったが、そもそも基本となる一様分布乱数生成で何度もコピーしているから今更だろう
#[cfg(test)] mod mutable_member_cell;