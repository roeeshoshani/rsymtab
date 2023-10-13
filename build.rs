fn main() {
    let cwd = std::env::current_dir().unwrap();
    println!(
        "cargo:rustc-link-arg={}/src/link.lds",
        cwd.to_str().unwrap()
    );
}
