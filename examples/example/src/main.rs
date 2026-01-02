use scoped_css::scoped_css;
use scoped_css_core::ScopedStyles;

const CSS: ScopedStyles = scoped_css!("example.css");

fn main() {
    println!("{CSS:?}");
}
