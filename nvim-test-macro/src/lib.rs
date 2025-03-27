const TEST_IDENT: &str = "TEST_FUNC";

#[cfg(feature = "testing")]
#[proc_macro_attribute]
pub fn nvim_test(
    t1: proc_macro::TokenStream,
    t2: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};
    use stuff::{get_exit_call, test_hook};
    use syn::ItemFn;

    let mut func: ItemFn = syn::parse_macro_input!(t2 as ItemFn);

    let hook_func = test_hook(&func);
    let cdylib_ident = format_ident!("{}_{}", func.sig.ident, TEST_IDENT);
    let exit_call: TokenStream = get_exit_call(t1).into();
    let orig_ident = &func.sig.ident;
    let orig_attrs = core::mem::take(&mut func.attrs);
    quote! {
        #hook_func
        #( #orig_attrs )*
        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub extern "C" fn #cdylib_ident(state: *mut ()) -> libc::c_int {
            let _th = unsafe { unlock() };
            #func
            let func: fn() -> () = #orig_ident;
            func();
            #exit_call(String::from("qall!"), false).unwrap();
            return 0;
        }
    }
    .into()
}

#[cfg(feature = "testing")]
mod stuff {
    use proc_macro2::TokenStream;
    use quote::{ToTokens, format_ident, quote};
    use syn::{
        Abi, Attribute, FnArg, Ident, ItemFn, Path, ReturnType, Token,
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        spanned::Spanned,
        token::Comma,
    };

    use crate::TEST_IDENT;
    pub fn cdylib_path() -> TokenStream {
        quote! {
            crate::CDYLIB_TEST_PATH
        }
    }

    pub fn validate_test_func(func: &ItemFn) -> Result<(), TokenStream> {
        validate_test_func_abi(func.sig.abi.as_ref())?;
        validate_test_func_args(&func.sig.inputs)?;
        validate_test_func_ret(&func.sig.output)?;
        validate_test_func_attributes(&func.attrs)?;

        Ok(())
    }

    fn validate_test_func_abi(abi: Option<&Abi>) -> Result<(), TokenStream> {
        let err = match abi.as_ref() {
            Some(Abi { name, .. }) => match name {
                Some(name) => {
                    if name.value() != "C" {
                        compile_error("unsupported extern kind in test function")
                    } else {
                        return Ok(());
                    }
                }

                None => compile_error("test function must be declared with extern \"C\""),
            },
            None => compile_error("test function must be declared with extern token"),
        };

        Err(err)
    }

    fn validate_test_func_args(args: &Punctuated<FnArg, Comma>) -> Result<(), TokenStream> {
        if args.is_empty() {
            Ok(())
        } else {
            Err(compile_error("test function cannot take arguments"))
        }
    }

    fn validate_test_func_ret(ret: &ReturnType) -> Result<(), TokenStream> {
        Ok(())
    }

    fn validate_test_func_attributes(attrs: &[Attribute]) -> Result<(), TokenStream> {
        let err_fn = || {
            compile_error(
                "test function must be declared with the `#[unsafe(no_mangle)]` attribute",
            )
        };
        let Some(attr) = attrs.iter().find(|attr| {
            attr.path()
                .is_ident(&Ident::new("unsafe", attr.path().span()))
        }) else {
            return Err(err_fn());
        };

        let mut no_mangle = false;
        attr.parse_nested_meta(|meta| {
            no_mangle = meta.path.is_ident(&Ident::new("no_mangle", attr.span()));
            Ok(())
        })
        .map_err(|err| err.into_compile_error())?;

        if !no_mangle {
            return Err(err_fn());
        }

        Ok(())
    }

    fn compile_error(s: &str) -> TokenStream {
        quote! {
            ::std::compile_error!(#s)
        }
    }

    pub fn test_hook(func: &ItemFn) -> TokenStream {
        let ident = &func.sig.ident;
        let cdylib_ident = format_ident!("{}_{}", func.sig.ident, TEST_IDENT);
        let dylib_path = cdylib_path();
        quote! {
            #[test]
            fn #ident() {
                if let Err(err) = ::nvim_test::test_body(&*#dylib_path, stringify!(#cdylib_ident)) {
                    panic!("{}", err);
                }
            }
        }
    }

    struct AttributeArgs {
        with: Ident,
        path: Path,
    }
    impl Parse for AttributeArgs {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let with: Ident = input.parse()?;
            assert_eq!(with.to_string(), "exit_call");
            let _: Token![=] = input.parse()?;
            let path = input.parse()?;
            Ok(Self { with, path })
        }
    }
    pub fn get_exit_call(t: proc_macro::TokenStream) -> proc_macro::TokenStream {
        if t.is_empty() {
            quote! {
                ::nvimium::nvim_funcs::nvim_exec
            }
            .into()
        } else {
            let args: AttributeArgs = syn::parse_macro_input!(t as AttributeArgs);
            args.path.into_token_stream().into()
        }
    }
}
