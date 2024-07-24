// use proc_macro;

// // Procedural macro
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}

// -- Custom derive macro --
pub trait HelloMacro {
    fn hello_macro();
}
