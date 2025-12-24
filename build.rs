fn main() {
    println!("cargo::rerun-if-changed=src/corn.lalrpop");

    lalrpop::process_src().expect("Failed to parse grammar");
}
