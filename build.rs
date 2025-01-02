use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        // Directory containing the .lib file
        println!(
            "cargo:rustc-link-search=native=C:\\Users\\{}\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib",
            env::var("USERNAME").unwrap()
        );

        // Add your manifest directory's DLL search logic only if necessary
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut dll_dir = manifest_dir.clone();

        // Skip DLL handling if not required
        dll_dir.push("dll");
        if dll_dir.exists() {
            for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
                let entry_path = entry.expect("Invalid fs entry").path();
                let file_name_result = entry_path.file_name();
                let mut new_file_path = manifest_dir.clone();
                if let Some(file_name) = file_name_result {
                    let file_name = file_name.to_str().unwrap();
                    if file_name.ends_with(".dll") {
                        new_file_path.push(file_name);
                        std::fs::copy(&entry_path, new_file_path.as_path())
                            .expect("Can't copy from DLL dir");
                    }
                }
            }
        } else {
            eprintln!(
                "Warning: DLL directory does not exist: {}",
                dll_dir.display()
            );
        }
    }
}
