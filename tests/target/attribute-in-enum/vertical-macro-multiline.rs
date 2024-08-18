enum A {
    B {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    },

    #[multiline_macro_attribute(
        very_very_long_option1,
        very_very_long_option2,
        very_very_long_option3,
    )]
    C {
        a: usize,
    },
}
