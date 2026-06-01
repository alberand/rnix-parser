use rnix::ast::HasEntry;
use std::error::Error;

use rnix::ast;
use rnix::ast::AttrpathValue;
use rnix::{NixLanguage, SyntaxKind, SyntaxNode};
use rowan::{GreenNodeBuilder, Language};

fn create_string(value: &str) -> SyntaxNode {
    let mut builder = GreenNodeBuilder::new();
    builder.start_node(NixLanguage::kind_to_raw(SyntaxKind::NODE_STRING));
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_START), "\"");
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_CONTENT), value);
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_END), "\"");
    builder.finish_node();

    SyntaxNode::new_root(builder.finish())
}

fn create_attrset(items: Vec<(SyntaxNode, SyntaxNode)>) -> SyntaxNode {
    let mut builder = GreenNodeBuilder::new();
    for (path, value) in items {
        builder.start_node(NixLanguage::kind_to_raw(SyntaxKind::NODE_STRING));
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_START), "\"");
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_CONTENT), value);
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_END), "\"");
        builder.finish_node();
    }

    SyntaxNode::new_root(builder.finish())
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = r#"
let
  flake-compat = builtins.fetchTarball {
    url = "https://github.com/edolstra/flake-compat/archive/99f1c2157fba4bfe6211a321fd0ee43199025dbf.tar.gz";
    sha256 = "0x2jn3vrawwv9xp15674wjz9pixwjyj3j771izayl962zziivbx2";
  };
in
(import flake-compat {
  src = ./.;
  src.component.version.modules = "1.0-rc2";
}).shellNix.default
    "#;
    let tree = rnix::Root::parse(&content).tree();

    println!("Original tree");
    // Print tree with node
    // print!("{:#?}", tree);
    print!("{:#?}", tree);
    println!("");

    println!("Modified tree");
    // Create new node which is basically '"whops 69"'
    let mut builder = GreenNodeBuilder::new();
    builder.start_node(NixLanguage::kind_to_raw(SyntaxKind::NODE_STRING));
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_START), "\"");
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_CONTENT), "whops 69");
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_END), "\"");
    builder.finish_node();
    let new_str = SyntaxNode::new_root(builder.finish());

    let expr = tree.expr().unwrap();
    let letin = match expr {
        ast::Expr::LetIn(expr) => expr,
        _ => return Err("root isn't a letin".into()),
    };
    let expr = letin.entries();
    for entry in expr {
        if let ast::Entry::AttrpathValue(attrpath_value) = entry {
            if let Some(ast::Expr::Apply(apply)) = attrpath_value.value() {
                if let Some(ast::Expr::AttrSet(set)) = apply.argument() {
                    for ee in set.entries() {
                        match ee {
                            ast::Entry::AttrpathValue(expr) => {
                                println!("{}", expr.value().unwrap())
                            }
                            _ => return Err("root isn't a letin".into()),
                        };
                    }
                }
            }
        }
        // rnix::edit::insert() needs position, it can be created like this
        // let position = rnix::edit::Position::last_child_of(&right_node);
        // rnix::edit::replace(right_node, new_str.clone_for_update());
    }

    Ok(())
}
