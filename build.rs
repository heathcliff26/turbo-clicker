fn main() {
    let config = slint_build::CompilerConfiguration::default().with_style("fluent".to_string());
    slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint build failed");
}
