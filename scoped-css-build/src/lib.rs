use std::{
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use glob::glob;
use lightningcss::{
    css_modules::Config,
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet, ToCssResult},
};

/// # Example
///
/// ```rust
/// //! build.rs
///
/// fn main() {
///     compile_css("src/**/*.css", "assets/main.generated.css");
/// }
/// ```
pub fn compile_css(input_pattern: &str, output_path: impl AsRef<Path>) {
    println!("cargo:rerun-if-changed={input_pattern}");

    let minify: bool = cfg!(not(debug_assertions));

    let mut file: BufWriter<File> = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(output_path)
            .unwrap(),
    );

    // the paths are guaranteed to be given in alphabetical order
    // https://docs.rs/glob/latest/glob/fn.glob.html
    for entry in glob(input_pattern).unwrap() {
        let path: PathBuf = entry.unwrap();
        let css_source: String = fs::read_to_string(path).unwrap();
        let stylesheet: StyleSheet<'_, '_> = StyleSheet::parse(
            &css_source,
            ParserOptions {
                css_modules: Some(Config::default()),
                ..ParserOptions::default()
            },
        )
        .unwrap();
        let printer: ToCssResult = stylesheet
            .to_css(PrinterOptions {
                minify,
                ..Default::default()
            })
            .unwrap();

        file.write_all(printer.code.as_bytes()).unwrap();
    }
    file.flush().unwrap();
}
