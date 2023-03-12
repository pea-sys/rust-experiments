fn main() {
    let env_vars = vec![
        "CARGO_PKG_VERSION",
        "CARGO_HOME",
        "CARGO_TARGET_DIR",
        "CARGO",
        "CARGO_MANIFEST_DIR",
        "CARGO_CRATE_NAME",
    ];

    for key in env_vars {
        let value = std::env::var(key).unwrap_or(String::from(""));

        println!("{}={}", key, value);
    }
}
