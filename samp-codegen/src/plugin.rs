use proc_macro::TokenStream;
use quote::quote;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{bracketed, parse_macro_input, Block, Ident, Path, Result, Stmt, Token};

use crate::REG_PREFIX;

struct InitPlugin {
    natives_list: Option<Punctuated<Path, Token![,]>>,
    block: Vec<Stmt>,
}

impl Parse for InitPlugin {
    fn parse(input: ParseStream) -> Result<Self> {
        let natives_list = if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            if ident == "natives" {
                let _: Token![:] = input.parse()?;

                let content;
                let _ = bracketed!(content in input);
                let natives = content.parse_terminated(Path::parse)?;

                let _: Token![,] = input.parse()?;

                Some(natives)
            } else {
                None
            }
        } else {
            None
        };

        Ok(InitPlugin {
            natives_list,
            block: input.call(Block::parse_within)?,
        })
    }
}

pub fn create_plugin(input: TokenStream) -> TokenStream {
    let mut plugin = parse_macro_input!(input as InitPlugin);
    let block = &plugin.block;

    let natives: proc_macro2::TokenStream = plugin
        .natives_list
        .iter_mut()
        .flatten()
        .map(|path| {
            if let Some(mut last_part) = path.segments.last_mut() {
                last_part.value_mut().ident = Ident::new(
                    &format!("{}{}", REG_PREFIX, last_part.value().ident),
                    last_part.value().ident.span(),
                );
            }
            quote!(#path(),)
        })
        .collect();

    let generated = quote! {
        #[no_mangle]
        pub extern "system" fn Load(server_data: *const usize) -> i32 {
            samp::interlayer::load(server_data);
            return 1;
        }

        #[no_mangle]
        pub extern "system" fn Unload() {
            samp::interlayer::unload();
        }

        #[no_mangle]
        pub extern "system" fn AmxLoad(amx: *mut samp::raw::types::AMX) {
            let natives = vec![#natives];

            samp::interlayer::amx_load(amx, &natives);

            unsafe {
                // drop allocated memory
                let _ = natives.into_iter()
                    .map(|info| std::ffi::CString::from_raw(info.name as *mut _))
                    .collect::<Vec<_>>();
            }
        }

        #[no_mangle]
        pub extern "system" fn AmxUnload(amx: *mut samp::raw::types::AMX) {
            samp::interlayer::amx_unload(amx);
        }

        #[no_mangle]
        pub extern "system" fn Supports() -> u32 {
            let constructor = || {
                #(#block)*
            };

            samp::plugin::initialize(constructor);
            samp::interlayer::supports()
        }

        #[no_mangle]
        pub extern "system" fn ProcessTick() {
            samp::interlayer::process_tick();
        }
    };

    generated.into()
}
