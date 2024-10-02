use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn canopy_mod(input: TokenStream) -> TokenStream {
    let mod_ident = parse_macro_input!(input as Ident);

    quote! {
        mod __ctor {
            use super::#mod_ident;

            use ::canopy::prelude::clap::Parser;

            ::canopy::prelude::lazy_static::lazy_static! {
                static ref BUILD_HOOK: ::canopy::prelude::retour::GenericDetour<extern "cdecl" fn(*const (), *mut ::canopy::prelude::bevy_app::App)> = unsafe {
                    ::canopy::hook::<extern "cdecl" fn(*const (), *mut ::canopy::prelude::bevy_app::App)>(::canopy::BUILD_HOOK, None, build)
                };
                pub static ref ARGUMENTS: <#mod_ident as ::canopy::CanopyMod>::Arguments = <#mod_ident as ::canopy::CanopyMod>::Arguments::parse_from(::canopy::ARGUMENTS.iter());
                pub static ref CANOPY_MOD: #mod_ident = <#mod_ident as ::canopy::CanopyMod>::initialize(&ARGUMENTS);
            }

            extern "cdecl" fn build(plugin: *const (), app: *mut ::canopy::prelude::bevy_app::App) {
                BUILD_HOOK.call(plugin, app);

                let app = unsafe { &mut *app };

                if let Err(err) = <#mod_ident as ::canopy::CanopyMod>::build(&*CANOPY_MOD, &*ARGUMENTS, app) {
                    ::canopy::prelude::tracing::error!("Could not initialize mod: {err}");
                }
            }

            #[::canopy::prelude::ctor::ctor]
            fn ctor() {
                ::canopy::initialize_logging();

                unsafe {
                    if let Err(err) = BUILD_HOOK.enable() {
                        ::canopy::prelude::tracing::error!("Could not enable build hook: {err}");
                    }
                }
            }
        }
    }.into()
}
