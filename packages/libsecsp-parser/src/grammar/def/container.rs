use std::str::FromStr;

use crate::grammar::atom;
use crate::grammar::block;
use crate::parser::Parser;
use crate::syntax::{KeywordKind};
use crate::syntax::SyntaxKind::*;

pub(crate) fn container(p: &mut Parser) {
    let m = p.mark();
    let is_abstract = p.eat_keyword(kw!["abstract"]);
    let kw = KeywordKind::from_str(p.current_text());

    match kw {
        Ok(kw!["block"]) | Ok(kw!["optional"]) | Ok(kw!["in"]) => {
            let kw = kw.unwrap();

            if is_abstract && (kw == kw!["optional"] || kw == kw!["in"]) {
                p.error("only blocks can be declared as abstract");
            }

            p.bump_as(kw);
        }
        _ => p.error("expected block or optional"),
    };

    p.expect(TOK_NAME);

    if p.at_text(kw!["extends"]) {
        atom::parse_extends_list(p);
    }

    block::parse_block(p);
    m.complete(p, NODE_CONTAINER_DEF);
}