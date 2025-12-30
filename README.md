# rust-i18n-derive

[![Crates.io](https://img.shields.io/crates/v/rust-i18n-derive.svg)](https://crates.io/crates/rust-i18n-derive)
[![Docs](https://docs.rs/rust-i18n-derive/badge.svg)](https://docs.rs/rust-i18n-derive)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**A concise, type-safe derive macro for [rust-i18n](https://github.com/longbridgeapp/rust-i18n) key mapping.**

`rust-i18n-derive` provides a convenient way to bind Rust Enums to your translation keys. Instead of using raw string literals (like `"menu.open"`) throughout your codebase, you can map them once in an Enum definition and use the generated methods to retrieve keys or localized strings.

This crate works alongside `rust-i18n`, acting as a bridge between strongly typed Rust structures and localization files.

## 🚀 Features

-   **Type-Safe Keys:** Use Enum variants (e.g., `Label::Login`) instead of error-prone string literals.
-   **Centralized Mapping:** Keep the association between code and translation keys in one place.
-   **Compile-time Enforcement:** The macro ensures that every enum variant has a mapped key via `#[tag("...")]`.
-   **Helper Methods:** Automatically implements `.key()` and `.localize()` methods for your Enums.

## 📖 Usage

1. **Setup `rust-i18n`**

    Ensure you have initialized `rust-i18n` in your project root (usually in `lib.rs` or `main.rs`), pointing to your locales directory:

    ```rust
    // Load I18n macro (from rust-i18n crate)
    rust_i18n::i18n!("locales");
    ```

2. **Derive `Localized`**

   Import the trait and derive macro, then annotate your enum variants with `#[tag("...")]`.
    ```rust
    use rust_i18n_derive::Localized;
    
    #[derive(Debug, Localized)]
    pub enum MainMenu {
        #[tag("menu.file.open")]
        Open,
    
        #[tag("menu.file.save")]
        Save,
    
        #[tag("menu.help.about")]
        About,
    }
    
    fn main() {
        let item = MainMenu::Open;
    
        // 1. Get the raw key (useful for logging or debugging)
        assert_eq!(item.key(), "menu.file.open");
    
        // 2. Get the localized string (uses rust_i18n::t! under the hood)
        // Assuming you have a locale file where "menu.file.open" = "Open File"
        println!("{}", item.localize()); 
    }
    ```

## 🛠 How it works

The macro generates an implementation of the `Localized` trait for your Enum.

For the example above, it generates code equivalent to:

```rust
impl Localized for MainMenu {
    fn key(&self) -> &'static str {
        match self {
            Self::Open => "menu.file.open",
            Self::Save => "menu.file.save",
            Self::About => "menu.help.about",
        }
    }

    fn localize(&self) -> String {
        rust_i18n::t!(self.key())
    }
}
```

## ⚠️ Error Handling

The macro validates your code at compile time. If you forget to add a #[tag("...")] attribute to a variant, the compiler will panic with a helpful error message pointing exactly to the missing line:

```txt
error: Missing #[tag("...")] for variant `Close`
  --> src/main.rs:15:5
   |
15 |     Close,
   |     ^^^^^
```

## 🤝 Contribution

Contributions are welcome! Please feel free to submit a Pull Request or open an issue on [GitHub](https://github.com/xairaven/rust-i18n-derive).

## 📄 License

This project is licensed under the [MIT License](https://www.google.com/search?q=MIT+License).