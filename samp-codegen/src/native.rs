use proc_macro::TokenStream;
use quote::quote;

use syn::parse::{Parse, ParseStream};
use syn::{Result, Ident, Error, Token, LitStr, parse_macro_input, ItemFn};

use crate::NATIVE_PREFIX;
use crate::REG_PREFIX;

struct NativeName {
    pub name: String,
    pub raw: bool,
}

impl Parse for NativeName {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut name = String::new();
        let mut raw = false;
        
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            
            if ident == "name" {
                let _: Token![=] = input.parse()?;
                let native_name: LitStr = input.parse()?;

                name = native_name.value();
            } else if ident == "raw" {
                raw = true;
            } else {
                return Err(Error::new(ident.span(), "Unexpected argument name. Currently supported only \"name\" and \"raw\"."));
            }

            let _: Option<Token![,]> = input.parse()?;
        }

        Ok(NativeName {
            name,
            raw,
        })
    }
}

pub fn create_native(args: TokenStream, input: TokenStream) -> TokenStream {
    let native = parse_macro_input!(args as NativeName);
    let origin_fn = parse_macro_input!(input as ItemFn);

    let vis = &origin_fn.vis;
    let origin_name = &origin_fn.ident;
    let native_name = prepend(&origin_fn.ident, NATIVE_PREFIX);
    let reg_name = prepend(&origin_fn.ident, REG_PREFIX);
    let amx_name = &native.name;

    let native_generated = quote! {
        #vis extern "C" fn #native_name(_: *mut samp::raw::types::AMX, _: *mut i32) -> i32 {
            let mut plugin = samp::plugin::get::<Self>();
            
            unsafe {
                plugin.as_mut().#origin_name(0, 0);
            }

            return 0;
        }
    };
    
    let reg_native = quote! {
        #vis fn #reg_name() -> samp::raw::types::AMX_NATIVE_INFO {
            samp::raw::types::AMX_NATIVE_INFO {
                name: std::ffi::CString::new(#amx_name).unwrap().into_raw(), 
                func: Self::#native_name,
            }
        }
    };

    let generated = quote! {
        #origin_fn
        #reg_native
        #native_generated
    };

    generated.into()
}

fn prepend(ident: &Ident, prefix: &str) -> Ident {
    Ident::new(&format!("{}{}", prefix, ident), ident.span())
}