use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::Ident;

use crate::types::{Action, Message, Service};

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub messages: Vec<Message>,
    pub services: Vec<Service>,
    pub actions: Vec<Action>,
}

impl Package {
    pub const fn new(name: String) -> Self {
        Self {
            name,
            messages: Vec::new(),
            services: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty() && self.services.is_empty() && self.actions.is_empty()
    }

    fn message_structs(&self, package_name: Ident, gen_cxx_bridge: bool) -> impl ToTokens {
        if self.messages.is_empty() {
            quote! {
                // empty msg
            }
        } else {
            let items = self
                .messages
                .iter()
                .map(|v| v.struct_token_stream(&package_name, gen_cxx_bridge));
            quote! {
                #(#items)*
            }
        }
    }

    fn message_aliases(&self, package_name: &Ident) -> impl ToTokens {
        if self.messages.is_empty() {
            quote! {
                // empty msg
            }
        } else {
            let items = self
                .messages
                .iter()
                .map(|v| v.alias_token_stream(package_name));
            quote! {
                pub mod msg {
                    #(#items)*
                }
            }
        }
    }

    fn messages_block(&self, gen_cxx_bridge: bool) -> impl ToTokens {
        if self.messages.is_empty() {
            quote! {
                // empty msg
            }
        } else {
            let items = self
                .messages
                .iter()
                .map(|v| v.token_stream_with_mod(gen_cxx_bridge));
            quote! {
                pub mod msg {
                    #(#items)*
                }  // msg
            }
        }
    }

    fn services_block(&self) -> impl ToTokens {
        if self.services.is_empty() {
            quote! {
                // empty srv
            }
        } else {
            let items = self.services.iter().map(|v| v.token_stream_with_mod());
            quote! {
                pub mod srv {
                    #(#items)*
                }  // srv
            }
        }
    }

    fn actions_block(&self) -> impl ToTokens {
        if self.actions.is_empty() {
            quote! {
                // empty srv
            }
        } else {
            let items = self.actions.iter().map(|v| v.token_stream_with_mod());
            quote! {
                pub mod action {
                    #(#items)*
                }  // action
            }
        }
    }

    pub fn struct_token_stream(&self, gen_cxx_bridge: bool) -> impl ToTokens {
        let package_name = Ident::new(&self.name, Span::call_site());
        let message_structs = self.message_structs(package_name, gen_cxx_bridge);

        quote! {
            #message_structs
        }
    }

    pub fn aliases_token_stream(&self) -> impl ToTokens {
        let package_name = Ident::new(&self.name, Span::call_site());
        let aliases = self.message_aliases(&package_name);

        quote! {
            pub mod #package_name {
                #aliases
            }
        }
    }

    pub fn token_stream(&self, gen_cxx_bridge: bool) -> impl ToTokens {
        let name = Ident::new(&self.name, Span::call_site());
        let messages_block = self.messages_block(gen_cxx_bridge);
        let services_block = self.services_block();
        let actions_block = self.actions_block();

        quote! {
            pub mod #name {
                #messages_block
                #services_block
                #actions_block
            }
        }
    }
}
