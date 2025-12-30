/// A derive macro to automatically implement localization methods for enums.
///
/// # Example
/// ```rust
/// use rust_i18n_derive::Localized;
///
/// #[derive(Localized)]
/// enum Message {
///     #[tag("msg.hello")]
///     Hello,
/// }
///
pub use rust_i18n_derive_impl::Localized;

pub trait Localized {
    fn key(&self) -> &'static str;
    fn localize(&self) -> std::borrow::Cow<'static, str>;
}
