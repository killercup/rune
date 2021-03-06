use crate::ast;
use crate::{Parse, ParseError, Parser};
use runestick::Span;

/// A block of expressions.
#[derive(Debug, Clone)]
pub struct ExprBlock {
    /// The close brace.
    pub block: ast::Block,
}

into_tokens!(ExprBlock { block });

impl ExprBlock {
    /// Get the span of the block.
    pub fn span(&self) -> Span {
        self.block.span()
    }

    /// Test if the block expression doesn't produce a value.
    pub fn produces_nothing(&self) -> bool {
        self.block.produces_nothing()
    }

    /// Test if the block is a constant expression.
    pub fn is_const(&self) -> bool {
        self.block.is_const()
    }
}

impl Parse for ExprBlock {
    fn parse(parser: &mut Parser<'_>) -> Result<Self, ParseError> {
        Ok(Self {
            block: parser.parse()?,
        })
    }
}
