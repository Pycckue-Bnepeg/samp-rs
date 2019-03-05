use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Error, FnArg, Ident, ItemFn, LitStr, Pat, Result, Token};

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
                return Err(Error::new(
                    ident.span(),
                    "Unexpected argument name. Currently supports only \"name\" and \"raw\".",
                ));
            }

            let _: Option<Token![,]> = input.parse()?;
        }

        Ok(NativeName { name, raw })
    }
}

// TODO: Allow use with functions that's not methods
pub fn create_native(args: TokenStream, input: TokenStream) -> TokenStream {
    let native = parse_macro_input!(args as NativeName);
    let origin_fn = parse_macro_input!(input as ItemFn);

    let vis = &origin_fn.vis;
    let origin_name = &origin_fn.ident;
    let args = origin_fn.decl.inputs.iter();
    let native_name = prepend(&origin_fn.ident, NATIVE_PREFIX);
    let reg_name = prepend(&origin_fn.ident, REG_PREFIX);
    let amx_name = &native.name;

    let fn_input = origin_fn.decl.inputs.iter().skip(2);

    let fn_input = fn_input
        .map(|arg| match arg {
            FnArg::Captured(capt) => {
                let pat = &capt.pat;

                if let Pat::Ident(pat_ident) = pat {
                    let ident = &pat_ident.ident;
                    Some(quote_spanned!(capt.span() => #ident))
                } else {
                    None
                }
            }
            _ => None,
        })
        .flatten();

    let args_parsing: proc_macro2::TokenStream = if !native.raw {
        args.skip(2).map(|arg| {
            match arg {
                FnArg::Captured(capt) => {
                    let pat = &capt.pat;

                    if let Pat::Ident(pat_ident) = pat {
                        let ident = &pat_ident.ident;
                        Some(quote_spanned!{
                            capt.span() => 
                                let #ident = match args.next() {
                                    Some(#ident) => #ident,
                                    None => {
                                        println!("error: couldn't parse variable {:?} in {:?} function.", stringify!(#ident), #amx_name);
                                        return 0;
                                    }
                                };
                        })
                    } else {
                        None
                    }
                },
                _ => None,
            }
        }).flatten().collect()
    } else {
        proc_macro2::TokenStream::new()
    };

    let call_origin = if !native.raw {
        quote!(plugin.as_mut().#origin_name(amx, #(#fn_input),*))
    } else {
        quote!(plugin.as_mut().#origin_name(amx, args))
    };

    let native_generated = quote! {
        #vis extern "C" fn #native_name(amx: *mut samp::raw::types::AMX, args: *mut i32) -> i32 {
            let amx_ident = samp::amx::AmxIdent::from(amx);

            let amx = match samp::amx::get(amx_ident) {
                Some(amx) => amx,
                None => {
                    return 0;
                }
            };

            let mut args = samp::args::Args::new(amx, args);
            let mut plugin = samp::plugin::get::<Self>();

            #(#args_parsing)*

            unsafe {
                match #call_origin {
                    Ok(retval) => {
                        return samp::plugin::convert_return_value(retval);
                    },

                    Err(err) => {
                        println!("error: {}", err);
                        return 0;
                    }
                }
            }
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
