use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Debug,
    lazy::SyncLazy,
    sync::Mutex,
};

use crate::{ASTKind, ASTNode, Factorial, FactorialFast, Primitive, Result, SMError, Span, Symbolic};

pub type FunctionFastPaths = BTreeMap<&'static str, fn(Span, &[ASTNode]) -> Result<ASTNode>>;

pub static FACTOR_INTEGER: SyncLazy<Mutex<FunctionFastPaths>> = SyncLazy::new(|| {
    let mut base: FunctionFastPaths = BTreeMap::new();
    base.insert("Factorial", Factorial::factor_integer_fast);
    Mutex::new(base)
});

pub trait FactorIntegerFast {
    fn factor_integer_fast(span: Span, args: &[ASTNode]) -> Result<ASTNode>;
}

impl FactorIntegerFast for Factorial {
    fn factor_integer_fast(span: Span, args: &[ASTNode]) -> Result<ASTNode> {
        match &args[0].kind {
            ASTKind::Atomic { atom } => atom.factorial(span, args[0].span),
            ASTKind::List { .. } => {}
            ASTKind::Function { .. } => {}
        }

        let out = ASTNode::primitive(2, span);
        Ok(out)
    }
}

#[derive(Debug, Clone)]
pub struct FactorInteger {}

impl FactorInteger {
    // pub fn new_fast(&mut self, name: &'static str, f: impl Fn(Span, &[ASTNode]) -> Result<ASTNode> + 'static) -> Result<()> {
    //     FACTOR_INTEGER.lock()?.insert(name, f);
    //     Ok(())
    // }
    pub fn try_fast(&self, name: &str, span: Span, args: &[ASTNode]) -> Result<ASTNode> {
        match FACTOR_INTEGER.lock()?.get(name) {
            None => Err(SMError::new(span, format!("Unknown function: {}", name))),
            Some(f) => f(span, args),
        }
    }
}

impl Symbolic for FactorInteger {
    fn name(&self) -> &'static str {
        "FactorInteger"
    }

    fn forward(&self, span: Span, args: &[ASTNode]) -> Result<ASTNode> {
        let out = match &args[0].kind {
            ASTKind::Atomic { atom } => self.try_fast(atom.name(), span, &[]).or_else(|_| atom.forward(span, &[]))?,
            ASTKind::List { items } => {
                let mut out = VecDeque::with_capacity(items.len());
                for item in items {
                    out.push_back(item.forward()?);
                }
                ASTNode { kind: ASTKind::List { items: out }, span }
            }
            ASTKind::Function { head, rest } => self.try_fast(head.name(), span, rest).or_else(|_| head.forward(span, rest))?,
        };
        Ok(out)
    }
}
