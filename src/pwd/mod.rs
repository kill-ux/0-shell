use std::path::PathBuf;

/// Print the current working directory.
///
/// # Parameters
/// - `current_dir`: the path to print.
///
/// # Returns
/// - `0` always.
pub fn pwd(current_dir: &PathBuf) -> i32 {
    println!("\x1b[1;34m{}\x1b[0m", current_dir.display());
    0
}
