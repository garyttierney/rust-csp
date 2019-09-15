extern crate itertools;
extern crate logos;
extern crate num_enum;
extern crate rowan;
extern crate secsp_parser;
extern crate secsp_syntax_derive;
extern crate smol_str;
extern crate text_unit;

pub mod ast;

mod parsing;
mod token;

pub use ast::SourceFile;
pub use parsing::Parse;

use ast::AstNode;
use rowan::{GreenNode, SyntaxNode};
use secsp_parser::ParseError;

impl SourceFile {
    pub fn parse<T: AsRef<str>>(text: T) -> Parse<SourceFile> {
        parsing::parse_text(text)
    }
}

#[test]
fn parse_source_file() {
    let sf = SourceFile::parse("block abc { type t; }");
}