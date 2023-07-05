use std::cell::Cell;

// インスタンスにmutを付与せずに、メンバー変数を変更できるかを試す構造体
#[cfg(test)]
struct TestMutableField {
    member_mut: Cell<u32>,
}

#[cfg(test)]
impl TestMutableField {
    // コンストラクタ
    fn new() -> Self {
        Self {
            member_mut: Cell::new(0u32),
        }
    }

    // メンバー変数の変更
    fn change_member(&self, arg: u32) {
        self.member_mut.set(arg);
    }

    // assert_eqの確認用
    fn get_member(&self) -> u32 {
        self.member_mut.get()
    }
}

#[test]
fn test_mutable_field_cell() {
    // 余談 void(?)を変数に束縛するとユニット構造体 () として扱われるようだ。
    // この後の式がFailになるので、コメントアウトしておく
    // let tester = TestMutableField::new().change_member(2u32);

    // フィールドを変更したい場合、素直にCellを用いるのが良さそうだ
    let tester = TestMutableField::new();
    assert_eq!(tester.get_member(), 0u32);

    tester.change_member(2u32);
    assert_eq!(tester.get_member(), 2u32);
}
