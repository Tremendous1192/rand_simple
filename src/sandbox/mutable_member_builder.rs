// インスタンスにmutを付与せずに、メンバー変数を変更できるかを試す構造体
#[cfg(test)]
struct TestMutableField {
    member_mut: u32,
}

#[cfg(test)]
impl TestMutableField {
    // コンストラクタ
    fn new() -> Self {
        Self {
            member_mut: 0u32,
        }
    }

    // メンバー変数の変更
    // インスタンスにmutを付けなくて済むが、戻り値がSelfのため、フィールド更新の度にシャドウイングが必要になる
    fn set_member(mut self, arg: u32) -> Self {
        self.member_mut = arg;
        self
    }

    // メンバー変数の変更
    // インスタンス生成時にmutを付与する必要がある
    fn change_member(&mut self, arg: u32) {
        self.member_mut = arg;
    }

    // assert_eqの確認用
    fn get_member(&self) -> u32 {
        self.member_mut.clone()
    }
}

#[test]
fn test_mutable_field_builder() {
    // この2行は思った通りに書けている
    // メンバー変数変更のメソッドをつなげて、インスタンスにmutを付与せずに、メンバー変数を変更できた
    let tester = TestMutableField::new().set_member(2u32);
    assert_eq!(tester.get_member(), 2u32);

    // この下の書き方は微妙だった
    // 戻り値にSelfを指定しているせいでライフタイムを消費してしまうため、シャドウイングを行う必要がある
    let tester = tester.set_member(1u32);
    assert_eq!(tester.get_member(), 1u32);

    // 下記のコードは、mutを付けなさいと怒られてコンパイルエラーになる
    //tester.change_member(3u32);

    // シャドウイングなしでメンバー変数を変更したい場合、インスタンス生成時にmutを付ける必要がある
    // ユーザーにmut付与を強いるのは、便利ではない
    let mut tester2 = TestMutableField::new();
    tester2.change_member(3u32);
    assert_eq!(tester2.get_member(), 3u32);
}