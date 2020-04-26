use std::io::Write;

fn generate_tests() {
    println!("cargo:rerun-if-changed=tests");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let out_path = std::path::Path::new(&out_dir).join("generated_tests.rs");
    let mut out_file = std::fs::File::create(out_path).unwrap();

    for entry in std::fs::read_dir("tests").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() && path.to_str().unwrap().starts_with("tests/test_") {
            write!(
                out_file,
                "
                    #[test]
                    fn {name}() {{
                        util::run_test(\"{name}\")
                    }}
                ",
                name = path.strip_prefix("tests").unwrap().to_str().unwrap(),
            )
            .unwrap();
        }
    }
}

fn main() {
    generate_tests()
}
