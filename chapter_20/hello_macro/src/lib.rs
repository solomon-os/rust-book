pub trait HelloMacro {
    fn hello_macro();
}

// Re-export the derive macro so consumers get the trait and the derive
// from one crate. Traits and macros live in different namespaces, so
// both can be named `HelloMacro` and one `use` imports the pair.
pub use hello_macro_derive::HelloMacro;
