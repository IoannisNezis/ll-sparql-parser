mod generator;
fn main() {
    if std::env::var("GENERATE_PARSER").is_ok() {
        generator::generate();
    }
}
