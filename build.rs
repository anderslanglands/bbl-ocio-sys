use cmake::Config;
use std::path::Path;
use regex::Regex;

#[cfg(target_os = "windows")]
fn print_link_args(dst: &Path) {
    // get linker args from LinkLine.txt cmake output
    // TODO: wrap this all up in a crate
    let linkline_fn = format!("{}/build/opencolorio-link-libraries.txt", dst.display());
    let contents = std::fs::read_to_string(&linkline_fn)
        .unwrap_or_else(|e| panic!("could not read {linkline_fn}: {e}"));

    println!("cargo:rustc-link-search=native={}/build", dst.display());

    for token in contents.split_whitespace() {
        let path = Path::new(&token.replace('\\', "/")).to_owned();
        if let Some(parent) = path.parent() {
            if !parent.to_string_lossy().is_empty() {
                println!("cargo:rustc-link-search=native={}", parent.display());
            }
        }
        println!(
            "cargo:rustc-link-lib={}",
            path.file_stem().unwrap().to_string_lossy()
        );
    }
}

#[cfg(target_os = "linux")]
fn print_link_args(dst: &Path) {
    // get linker args from LinkLine.txt cmake output
    // TODO: wrap this all up in a crate
    let linkline_fn = format!("{}/build/opencolorio-link-libraries.txt", dst.display());
    let contents = std::fs::read_to_string(&linkline_fn)
        .unwrap_or_else(|e| panic!("could not read {linkline_fn}: {e}"));

    println!("cargo:rustc-link-search=native={}/build", dst.display());

    for token in contents.split_whitespace() {
        let path = Path::new(&token.replace('\\', "/")).to_owned();
        if let Some(parent) = path.parent() {
            if !parent.to_string_lossy().is_empty() {
                println!("cargo:rustc-link-search=native={}", parent.display());
            }
        }

        // stem will be either "libsomething" or "-lstdc++""
        let stem = path.file_stem().unwrap().to_string_lossy();

        let lib = if let Some(stripped) = stem.strip_prefix("-l") {
            stripped.to_string()
        } else if let Some(stripped) = stem.strip_prefix("lib") {
            if let Some(stripped) = stripped.strip_suffix(".a") {
                stripped.to_string()
            } else {
                let re = Regex::new(r"(.*)\.so[\.0-9]*|\.a").unwrap();
                if let Some(caps) = re.captures(stripped) {
                    caps[1].to_string()
                } else {
                    stripped.to_string()
                }
            }
        } else {
            panic!("unknown lib form \"{stem}\" from \"{}\"", path.display());
        };

        println!("cargo:rustc-link-lib={lib}");
    }
}

pub fn main() {
    let dst = Config::new("bbl-ocio")
        .generator("Ninja")
        .profile("Release")
        .build();

    print_link_args(&dst);
}
