//! 现在只支持tokio

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RuntimeKind {
    Tokio,
}