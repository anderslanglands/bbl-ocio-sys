use bbl_build::Config;

pub fn main() {
    let _dst = Config::new("opencolorio", "bbl-ocio")
        .generator("Ninja")
        .profile("Release")
        .define("BBL_LANGUAGES", "rust")
        .build();
}
