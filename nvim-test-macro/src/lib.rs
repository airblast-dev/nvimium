//! See nvim-test workspace for information on whats happening in this section

#[cfg(feature = "testing")]
#[proc_macro_attribute]
pub fn nvim_test(
    t1: proc_macro::TokenStream,
    t2: proc_macro::TokenStream,
) -> proc_macro::TokenStream {

    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};
    use stuff::{get_exit_call, test_hook};
    use syn::{spanned::Spanned, ItemFn};

    let mut func: ItemFn = syn::parse_macro_input!(t2 as ItemFn);

    let fs = func.span()
    let start = fs.start();
    let end = fs.end();
    let hook_func = test_hook(&func);
    // generate an extremely ugly name to minimize collision chances
    let cdylib_ident = format_ident!("_____{}_{}_ls{}_sc{}_le{}_ce{}", func.sig.ident, "TEST_FUNC", start.line, start.column, end.line, end.column);
    let exit_call: TokenStream = get_exit_call(t1).into();
    let orig_ident = &func.sig.ident;
    let orig_attrs = core::mem::take(&mut func.attrs);

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
        pub extern "C" fn #cdylib_ident(state: *mut ()) -> ::std::ffi::c_int {
            let _th = unsafe { nvim_test::thread_lock::unlock() };
            let panic_out_th = nvim_funcs::global::nvim_get_var(c"NVIMIUM_PANIC_LOG_FILE").unwrap().into_string().unwrap();
            let panic_out_path = ::std::path::PathBuf::from(::std::string::String::from_utf8(panic_out_th.as_thinstr().as_slice().to_vec()).unwrap());
            nvim_test::set_test_panic_hook(panic_out_path);
            #func
            let func: fn() -> () = #orig_ident;
            func();
            #exit_call;
            return 0;
        }
    }
    .into()
}

#[cfg(feature = "testing")]
mod stuff {
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};
    use syn::{
        Ident, ItemFn, Path, Token,
        parse::{Parse, ParseStream},
    };

    pub fn cdylib_path() -> TokenStream {
        quote! {
            crate::CDYLIB_TEST_PATH
        }
    }

    pub fn test_hook(func: &ItemFn) -> TokenStream {
        let ident = &func.sig.ident;
        let cdylib_ident = format_ident!("{}_{}", func.sig.ident, "TEST_FUNC");
        let dylib_path = cdylib_path();
        quote! {
            fn #ident() {
                if let Err(err) = nvim_test::test_body(&*#dylib_path, stringify!(#cdylib_ident)) {
                    panic!("{}", err);
                }
            }
        }
    }

    struct AttributeArgs {
        path: Path,
    }
    impl Parse for AttributeArgs {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let with: Ident = input.parse()?;
            assert_eq!(with.to_string(), "exit_call");
            let _: Token![=] = input.parse()?;
            let path = input.parse()?;
            Ok(Self { path })
        }
    }
    pub fn get_exit_call(t: proc_macro::TokenStream) -> proc_macro::TokenStream {
        if t.is_empty() {
            quote! {
                nvim_funcs::vimscript::nvim_exec2(c":qall!", &Default::default()).unwrap()
            }
            .into()
        } else {
            let args: AttributeArgs = syn::parse_macro_input!(t as AttributeArgs);
            let path = args.path;
            quote! { #path() }.into()
        }
    }
}
