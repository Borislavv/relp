#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Repeat {
    Once,
    Always,
    Times(i64),
}
