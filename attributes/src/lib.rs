use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn say_hello(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_sig = &input.sig;

    let expanded = quote! {
        #fn_sig {
            println!("Fonksiyon çalışıyor: {}", stringify!(#fn_name));
            #fn_block
        }
    };

    TokenStream::from(expanded)
}
