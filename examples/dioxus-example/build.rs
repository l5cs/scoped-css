use scoped_css_build::compile_css;

fn main() {
    compile_css("src/**/*.css", "assets/main.generated.css");
}
