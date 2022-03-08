use std::collections::VecDeque;

use crate::{ASTKind, ASTNode, Span};

impl ASTNode {
    pub fn list<I>(items: I, span: Span) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        Self { kind: ASTKind::List { items: VecDeque::from_iter(items) }, span }
    }
}
