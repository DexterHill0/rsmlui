mod doc_gen;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rmldoc(attrs: TokenStream, item: TokenStream) -> TokenStream {
    doc_gen::doc_gen(attrs, item)
}
