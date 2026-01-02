use scoped_css::{ScopedStyles, scoped_css};

const CSS: ScopedStyles = scoped_css!("example.css");

fn main() {
    println!("{CSS:?}");
}
