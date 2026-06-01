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

fn create_attrpath(value: &str) -> SyntaxNode {
    let mut builder = GreenNodeBuilder::new();
    builder.start_node(NixLanguage::kind_to_raw(SyntaxKind::NODE_ATTRPATH_VALUE));
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::NODE_ATTRPATH), "attr_path");
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_IDENT), value);
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_WHITESPACE), " ");
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_ASSIGN), "=");
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_WHITESPACE), " ");
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::NODE_PATH), "node_path");
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_PATH), "../");
    builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_SEMICOLON), ";");
    builder.finish_node();

    SyntaxNode::new_root(builder.finish())
}

fn create_attrset(items: Vec<(SyntaxNode, SyntaxNode)>) -> SyntaxNode {
    let mut builder = GreenNodeBuilder::new();
    for (path, value) in items {
        builder.start_node(NixLanguage::kind_to_raw(SyntaxKind::NODE_STRING));
        builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_START), "\"");
        builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_CONTENT), "set value");
        builder.token(NixLanguage::kind_to_raw(SyntaxKind::TOKEN_STRING_END), "\"");
        builder.finish_node();
    }

    SyntaxNode::new_root(builder.finish())
}

fn main() -> Result<(), Box<dyn Error>> {
    let node = create_string("wopsiii");

    println!("Created string");
    print!("{:#?}", node);
    println!("");

    let node = create_attrpath("attribute");

    println!("Created attrpath");
    print!("{:#?}", node);
    print!("{}", node);
    println!("");

    /*let items = vec![("version", "1.0"), ("name", "popi-package")];
    let node = create_attrset(items);

    println!("Created attrset");
    print!("{:#?}", node);
    println!("");*/

    Ok(())
}
