pub fn main() {
    let _dst = bbl_build::Config::new("opencolorio", "bbl-ocio")
        .generator("Ninja")
        .profile("Release")
        .build();
}
