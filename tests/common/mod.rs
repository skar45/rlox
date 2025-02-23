use assert_cmd::Command;

const INTERPRETER: &'static str = "rlox";

pub fn setup() -> Command {
    Command::cargo_bin(INTERPRETER).unwrap()
}
