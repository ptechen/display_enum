# enum_display_derive

[![Version info](https://img.shields.io/crates/v/display_enum.svg)](https://crates.io/crates/display_enum)
[![Downloads](https://img.shields.io/crates/d/display_enum.svg?style=flat-square)](https://crates.io/crates/display_enum)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/display_enum)
[![dependency status](https://deps.rs/crate/display_enum/0.1.3/status.svg)](https://deps.rs/crate/display_enum)


# example:
```rust
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
```