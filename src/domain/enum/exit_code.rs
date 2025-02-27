#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ExitCode {
    Success,
    Failed,
    Wife,
    Other(i32),
}
