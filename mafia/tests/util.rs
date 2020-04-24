use std::io::Write;

pub fn save<T: serde::ser::Serialize, P: AsRef<std::path::Path>>(
    mint: &mut goldenfile::Mint,
    path: P,
    value: &T,
) {
    let config = ron::ser::PrettyConfig::default();
    let serialized_value = ron::ser::to_string_pretty(&value, config).unwrap();

    let mut output_file = mint.new_goldenfile(path).unwrap();
    write!(output_file, "{}", serialized_value).unwrap();
}
