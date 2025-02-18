mod generator;
fn main() {
    generator::generate();
    println!("cargo::rerun-if-changed=build.rs");
}
