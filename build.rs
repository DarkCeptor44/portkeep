use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=frontend/src");
    println!("cargo:rerun-if-changed=frontend/static");
    println!("cargo:rerun-if-changed=frontend/package.json");
    println!("cargo:rerun-if-changed=frontend/vite.config.ts");

    if std::env::var("PROFILE").unwrap() == "release" {
        let status = Command::new("bun")
            .args(["run", "build"])
            .current_dir("frontend")
            .status()
            .expect("Failed to execute bun build command");

        if !status.success() {
            panic!("Frontend build process failed");
        }
    }
}
