fn main() {
    let config = slint_build::CompilerConfiguration::default()
        .with_style("fluent".to_string())
        .with_bundled_translations("translations");
    slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint build failed");

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=translations");
    println!("cargo::rerun-if-changed=ui");
}
