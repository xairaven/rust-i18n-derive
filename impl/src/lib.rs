use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, LitStr};

#[proc_macro_derive(Localized, attributes(tag))]
pub fn localized_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Ensure we are working with an enum
    let variants = match input.data {
        Data::Enum(d) => d.variants,
        _ => panic!("Localized can only be derived for enums"),
    };

    // Generate match arms for each variant
    let match_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        // Find the #[tag("...")] attribute
        let tag_attr = variant.attrs.iter().find(|attr| attr.path().is_ident("tag"));

        let key_literal = if let Some(attr) = tag_attr {
            // Parse the string literal inside #[tag("...")]
            attr.parse_args::<LitStr>()
                .expect("tag attribute must contain a string literal, e.g., #[tag(\"key.name\")]")
                .value()
        } else {
            // Fallback: if no tag is provided, maybe panic or use the variant name?
            // For now, let's panic to enforce explicit mapping.
            panic!("Variant {} is missing #[tag(\"...\")] attribute", variant_ident);
        };

        // Generate the code for this variant
        quote! {
            Self::#variant_ident => #key_literal
        }
    });

    // Build the impl block
    // We assume the user has `rust_i18n` available in their crate.
    let expanded = quote! {
        impl Localized for #name {
            /// Returns the translation key associated with this variant.
            fn key(&self) -> &'static str {
                match self {
                    #(#match_arms),*
                }
            }

            /// Returns the localized string using rust_i18n::t! macro.
            fn localize(&self) -> String {
                rust_i18n::t!(self.key())
            }
        }
    };

    TokenStream::from(expanded)
}
