mod doc_gen;
mod sys_cast;
mod utils;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rmldoc(attrs: TokenStream, item: TokenStream) -> TokenStream {
    doc_gen::doc_gen(attrs, item)
}

#[proc_macro_attribute]
pub fn sys_cast(attrs: TokenStream, item: TokenStream) -> TokenStream {
    sys_cast::generate(attrs, item)
}

#[proc_macro]
pub fn sys_cast_external(item: TokenStream) -> TokenStream {
    todo!()
    // sys_cast::generate(attrs, item)
}
