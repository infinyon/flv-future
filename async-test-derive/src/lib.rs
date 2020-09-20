extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;
use syn::Ident;
use proc_macro2::Span;


#[proc_macro_attribute]
pub fn test_async(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let input = syn::parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let sync_name = format!("{}_sync",name);
    let out_fn_iden = Ident::new(&sync_name, Span::call_site());

    let expression = quote! {

        #[test]
        fn #out_fn_iden()  {

            ::flv_future_aio::subscriber::init_logger();
          
            #input
            
            let ft = async {
                #name().await
            };

            if let Err(err) = ::flv_future_aio::task::run_block_on(ft) {
                assert!(false,"error: {:?}",err);
            }            
            
        }
    };
    
    expression.into()
   
}