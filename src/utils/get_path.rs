///
/// Get the base path to save directory
///
pub fn get_base_path(arg: &str, base_path: &str) -> String {
    let mut base_path: String = String::from(base_path);
    base_path.push_str(arg);
    base_path.push_str("/pass");

    base_path
}
