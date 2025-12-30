use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, LitStr, parse_macro_input};

#[proc_macro_derive(Localized, attributes(tag))]
pub fn localized_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = match input.data {
        Data::Enum(d) => d.variants,
        _ => {
            return syn::Error::new_spanned(
                name,
                "Localized can only be derived for enums",
            )
            .to_compile_error()
            .into();
        },
    };

    // We will collect generated match arms here
    let mut match_arms = Vec::new();
    // We will collect any errors found here
    let mut errors = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;
        let tag_attr = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("tag"));

        if let Some(attr) = tag_attr {
            // Try to parse the attribute content
            match attr.parse_args::<LitStr>() {
                Ok(literal) => {
                    let value = literal.value();
                    match_arms.push(quote! {
                        Self::#variant_ident => #value
                    });
                },
                Err(e) => {
                    // Error if format is wrong, e.g. #[tag(123)] instead of string
                    errors.push(syn::Error::new_spanned(attr, e.to_string()));
                },
            }
        } else {
            // Error if tag is missing completely
            // new_spanned(variant) makes the error appear on the variant line itself
            errors.push(syn::Error::new_spanned(
                variant.clone(),
                format!("Missing #[tag(\"...\")] for variant `{}`", variant_ident),
            ));
        }
    }

    // If we found any errors, return them immediately.
    // This prints nice compiler errors pointing to the specific lines.
    if !errors.is_empty() {
        let error_tokens = errors.iter().map(|err| err.to_compile_error());
        return quote! {
            #(#error_tokens)*
        }
        .into();
    }

    // If no errors, generate the implementation
    let expanded = quote! {
        impl Localized for #name {
            /// Returns the translation key associated with this variant.
            fn key(&self) -> &'static str {
                match self {
                    #(#match_arms),*
                }
            }

            /// Returns the localized string using rust_i18n::t! macro.
            fn localize(&self) -> std::borrow::Cow<'static, str> {
                rust_i18n::t!(self.key())
            }
        }
    };

    TokenStream::from(expanded)
}
