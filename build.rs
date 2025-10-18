use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Force target triple to aarch64
    let forced_target = "aarch64-unknown-linux-gnu";
    let host = env::var("HOST").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:warning=Forcing target triple to {}", forced_target);

    // Inject a cfg for conditional compilation
    println!("cargo:rustc-cfg=feature=\"forced_aarch64\"");

    // Optional: generate a file with build info
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_info.rs");

    let contents = format!(
        r#"
        pub const TARGET: &str = "{forced_target}";
        pub const HOST: &str = "{host}";
        "#
    );

    fs::write(&dest_path, contents).unwrap();
    println!("cargo:rerun-if-changed={}", dest_path.display());
}
