pub use rust_i18n_derive_impl::Localized;

pub trait Localized {
    fn key(&self) -> &'static str;
    fn localize(&self) -> String;
}
