use std::{
    env,
    path::{Path, PathBuf},
};

/// Get the current input file based on the arguments of the program. If no argument
/// is passed, it will automatically grab a file called `input.txt` in the correct
/// directory.
///
/// You probably want to call this at the start.
pub fn get_input_file() -> PathBuf {
    let args: Vec<String> = env::args().collect();

    if let Some(file) = args.get(1) {
        Path::new(file).to_path_buf()
    } else {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        format!("{manifest_dir}/input.txt").into()
    }
}
