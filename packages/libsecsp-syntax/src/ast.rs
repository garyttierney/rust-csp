use std::marker::PhantomData;

use crate::ast::types::WalkEvent;

// Re-export AST types under the crate::ast root namespace.
pub use self::error::SyntaxError;
pub use self::kinds::*;
pub use self::types::{
    GreenNode, GreenNodeBuilder, SyntaxNode, SyntaxNodeChildren, SyntaxNodeRef, TreeArc,
};

#[macro_use]
pub mod types;

mod error;
mod kinds;
pub mod visitor;

pub trait AstNode:
    rowan::TransparentNewType<Repr = SyntaxNode> + ToOwned<Owned = TreeArc<Self>>
{
    fn cast(syntax: &SyntaxNode) -> Option<&Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;

    fn child<C: AstNode>(&self) -> &C {
        self.children().next().unwrap()
    }

    fn children<C: AstNode>(&self) -> AstChildren<C> {
        AstChildren::new(self.syntax())
    }
}

#[derive(Debug)]
pub struct AstChildren<'a, N> {
    inner: SyntaxNodeChildren<'a>,
    ph: PhantomData<N>,
}

impl<'a, N> AstChildren<'a, N> {
    fn new(parent: &'a SyntaxNode) -> Self {
        AstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<'a, N: AstNode + 'a> Iterator for AstChildren<'a, N> {
    type Item = &'a N;
    fn next(&mut self) -> Option<&'a N> {
        loop {
            if let Some(n) = N::cast(self.inner.next()?) {
                return Some(n);
            }
        }
    }
}

pub fn descendants(tree: SyntaxNodeRef) -> impl Iterator<Item = SyntaxNodeRef> {
    tree.preorder().filter_map(|event| match event {
        WalkEvent::Enter(node) => Some(node),
        WalkEvent::Leave(_) => None,
    })
}
