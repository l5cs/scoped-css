use std::{fs, path::PathBuf};

use lightningcss::{
    css_modules::Config,
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet, ToCssResult},
};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use unsynn::{IParse, LiteralString, ToTokens, unsynn};

unsynn! {
    struct PathLiteral(LiteralString);
}

// todo: proper compile time errors
#[proc_macro]
pub fn scoped_css(input: TokenStream) -> TokenStream {
    let path: PathLiteral = TokenStream2::from(input).into_token_iter().parse().unwrap();
    // todo: this panics in rust-analyzer
    let caller_file: PathBuf = Span::call_site().local_file().expect("call site");
    let css_path: PathBuf = caller_file.parent().unwrap().join(path.0.as_str());
    let css_source: String = fs::read_to_string(css_path).unwrap();

    let stylesheet: StyleSheet<'_, '_> = StyleSheet::parse(
        &css_source,
        ParserOptions {
            css_modules: Some(Config::default()),
            ..ParserOptions::default()
        },
    )
    .unwrap();
    // minifying here does nothing
    let printer: ToCssResult = stylesheet
        .to_css(PrinterOptions::default())
        .unwrap();

    let entries = printer
        .exports
        .unwrap()
        .into_iter()
        .map(|(original, export)| {
            let scoped: String = export.name;
            quote! { (#original, #scoped) }
        });

    quote! {
        ScopedStyles {
            classes: &[ #(#entries),* ],
        }
    }
    .into()
}
