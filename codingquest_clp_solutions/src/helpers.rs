use std::fs;
use std::path::PathBuf;

pub fn get_problem_input_data(problem: u32) -> crate::Result<String> {
    Ok(fs::read_to_string(
        [env!("CARGO_MANIFEST_DIR"), "input_data", &format!("problem_{problem}.txt")]
            .iter()
            .collect::<PathBuf>(),
    )?)
}
