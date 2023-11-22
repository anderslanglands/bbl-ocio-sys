use cmake::Config;
use std::path::Path;

pub fn main() {
    let dst = Config::new("bbl-ocio")
        .generator("Ninja")
        .profile("Release")
        .build();

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
        #[cfg(windows)]
        println!(
            "cargo:rustc-link-lib={}",
            path.file_stem().unwrap().to_string_lossy()
        );
        #[cfg(not(windows))]
        unimplemented!()
    }
}
