//! See nvim-test workspace for information on whats happening in this section

#[cfg(feature = "testing")]
#[proc_macro_attribute]
pub fn nvim_test(
    t1: proc_macro::TokenStream,
    t2: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote, quote_spanned};
    use stuff::{get_exit_call, test_hook};
    use syn::{ItemFn, spanned::Spanned};

    let mut func: ItemFn = syn::parse_macro_input!(t2 as ItemFn);

    let fs = func.span();
    let start = fs.start();
    let end = fs.end();
    let byte_range = fs.byte_range();
    // generate an extremely ugly name to minimize collision chances
    let cdylib_ident = format_ident!(
        "_____{}_{}_ls{}_sc{}_bs{}_le{}_ce{}_be{}",
        func.sig.ident,
        "TEST_FUNC",
        start.line,
        start.column,
        byte_range.start,
        end.line,
        end.column,
        byte_range.end
    );
    let hook_func = test_hook(&func.sig.ident, &cdylib_ident);
    let exit_call: TokenStream = get_exit_call(t1).into();
    let orig_ident = &func.sig.ident;
    let orig_attrs = core::mem::take(&mut func.attrs);
    let sp_quote = quote_spanned! {fs => #func};
    // TODO: provide better ways to set attributes on test funcs
    quote! {
        #[cfg(test)]
        #( #orig_attrs )*
        #[test]
        #hook_func
        // HACK: we cant remove the function without bogus unused imports
        // so just remove the no_mangle.
        #[cfg_attr(not(test), unsafe(no_mangle))]
        #[allow(non_snake_case)]
        #[doc(hidden)]
        pub extern "C" fn #cdylib_ident(l: *mut ()) -> ::std::ffi::c_int {
            #sp_quote
            unsafe { nvimium::test_macro_utils::test_body(l as *mut _, #orig_ident, #exit_call) }
        }
    }
    .into()
}

#[cfg(feature = "testing")]
mod stuff {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{
        Ident,
        parse::{Parse, ParseStream},
    };

    pub fn cdylib_path() -> TokenStream {
        quote! {
            crate::CDYLIB_TEST_PATH
        }
    }

    pub fn test_hook(real_ident: &Ident, cdylib_ident: &Ident) -> TokenStream {
        let dylib_path = cdylib_path();
        quote! {
            fn #real_ident() {
                if let Err(err) = nvimium::nvim_test::test_body(&*#dylib_path, stringify!(#cdylib_ident)) {
                    panic!("{}", err);
                }
            }
        }
    }

    struct AttributeArgs;
    impl Parse for AttributeArgs {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let with: Ident = input.parse()?;
            assert_eq!(with.to_string(), "no_exit");
            Ok(Self)
        }
    }
    pub fn get_exit_call(t: proc_macro::TokenStream) -> proc_macro::TokenStream {
        if t.is_empty() {
            quote! {
                (||{nvimium::nvim_funcs::vimscript::exec2(c":qall!", &Default::default()).unwrap(); })
            }
            .into()
        } else {
            let _: AttributeArgs = syn::parse_macro_input!(t as AttributeArgs);
            quote! {
                #[allow(unused)]
                || {}
            }
            .into()
        }
    }
}
