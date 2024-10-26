# enum_display_derive

## example:
    
    use display_enum::Display;

    #[derive(Display)]
    //#[ignore_field]
    enum Test{
        A(i32),
        #[ignore_field]
        B(Data, i32),
        TestA
    }
    
    #[derive(Debug)]
    struct Data {
        test: i32
    }

    #[test]
    fn test() {
        println!("{}", Test::B(Data{test: 5}, 6).to_string());
    }