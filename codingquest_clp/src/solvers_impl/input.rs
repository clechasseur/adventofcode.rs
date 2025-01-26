use std::fs;
use std::path::Path;

use aoclp::solvers_impl::input::Input;

pub fn get_input<S>(input: S) -> crate::Result<Input<'static>>
where
    S: Into<String>,
{
    Ok(Input::for_example(input))
}

pub fn get_input_from_file<P>(path: P) -> crate::Result<Input<'static>>
where
    P: AsRef<Path>,
{
    get_input(fs::read_to_string(&path)?)
}
