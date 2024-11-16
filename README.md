# enum_display_derive

[![Version info](https://img.shields.io/crates/v/display_enum.svg)](https://crates.io/crates/display_enum)
[![Downloads](https://img.shields.io/crates/d/display_enum.svg?style=flat-square)](https://crates.io/crates/display_enum)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/display_enum)
[![dependency status](https://deps.rs/crate/display_enum/0.1.5/status.svg)](https://deps.rs/crate/display_enum)


# example:
```rust
use display_enum::Display;

#[derive(Display)]
#[to_vec]
//#[ignore_field]
enum Test{
    A(i32),
    #[ignore_field]
    B(Data, i32),
    C(Data, i32),
    TestA,
    #[ignore_field]
    Data{name: Data, test: i64},
    Data1{name: Data, test: i64},
}

#[derive(Debug)]
struct Data {
    test: i32
}
#[test]
fn test() {
    assert_eq!("B", Test::B(Data{test: 5}, 6).to_string());
    assert_eq!("C:(Data { test: 5 }, 6)", Test::C(Data{test: 5}, 6).to_string());
    assert_eq!("Data", Test::Data {name: Data{ test: 0 }, test: 42}.to_string());
    assert_eq!("Data1:(Data { test: 0 }, 42)", Test::Data1 {name: Data{ test: 0 }, test: 42}.to_string());
}
```