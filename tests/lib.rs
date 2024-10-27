use display_enum::Display;

#[derive(Display)]
//#[ignore_field]
enum Test{
    A(i32),
    #[ignore_field]
    B(Data, i32),
    C(Data, i32),
    TestA
}

#[derive(Debug)]
struct Data {
    test: i32
}
#[test]
fn test() {
    assert_eq!("B", Test::B(Data{test: 5}, 6).to_string());
    assert_eq!("C:(Data { test: 5 }, 6)", Test::C(Data{test: 5}, 6).to_string());
}