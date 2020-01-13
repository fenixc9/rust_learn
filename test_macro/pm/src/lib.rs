#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


extern crate proc_macro;

use proc_macro::TokenStream;
extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro]
pub fn count_tt1(ts: TokenStream) -> TokenStream {
    ts.into_iter().count().to_string().parse().unwrap()
}

#[proc_macro_derive(HelloWorld)]
pub fn hw(ts: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(ts).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl HelloWorld for #name {
            fn hw(&self) -> () {
                println!("Hello, {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}