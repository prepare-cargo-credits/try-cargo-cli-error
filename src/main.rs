use cargo::util::CliError;
use cargo::core::Shell;

fn main() {
    let mut shell = Shell::new();
    cargo::exit_with_error(CliError::code(100), &mut shell);
}
