use std::fs;
use std::path::PathBuf;

pub fn get_problem_input_data(problem: u32) -> crate::Result<String> {
    get_problem_data(problem, "problem")
}

pub fn get_problem_example_data(problem: u32) -> crate::Result<String> {
    get_problem_data(problem, "example")
}

fn get_problem_data(problem: u32, prefix: &str) -> crate::Result<String> {
    Ok(fs::read_to_string(
        [env!("CARGO_MANIFEST_DIR"), "input_data", &format!("{prefix}_{problem}.txt")]
            .iter()
            .collect::<PathBuf>(),
    )?)
}
