framework agnostic scoped css for rust front-end frameworks. 

basically a wrapper around [`lightningcss`] css modules implementation. 

## installation

```bash
cargo add --git https://github.com/l5cs/scoped-css.git
cargo add --build --git ttps://github.com/l5cs/scoped-css.git --package scoped-css-build
```

## how to use

```rust
//! build.rs

use scoped_css_build::compile_css;

fn main() {
    compile_css("src/**/*.css", "assets/main.generated.css");
}
```

### dioxus example

```rust
use dioxus::prelude::*;
use scoped_css::{ScopedStyles, scoped_css};

const CSS_ASSET: Asset = asset!("/assets/main.generated.css");
const CSS: ScopedStyles = scoped_css!("main.css");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class: CSS["scoped"],
            p { class: CSS["p"], "hello css" }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS_ASSET }
        Hero {}
    }
}
```

## todo

- [ ] sass 
- [ ] useful compilation errors
- [ ] publish to crates.io?
- [ ] examples for other frameworks
