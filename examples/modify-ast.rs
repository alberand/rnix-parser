use rowan::{GreenNodeBuilder, Language};
use rnix::{NixLanguage, SyntaxNode, SyntaxKind};
use rowan::ast::AstNode;

fn main() {
    let content = r#"
let
  flake-compat = builtins.fetchTarball {
    url = "https://github.com/edolstra/flake-compat/archive/99f1c2157fba4bfe6211a321fd0ee43199025dbf.tar.gz";
    sha256 = "0x2jn3vrawwv9xp15674wjz9pixwjyj3j771izayl962zziivbx2";
  };
in
(import flake-compat {
  src = ./.;
}).shellNix.default
    "#;
    let tree = rnix::Root::parse(&content).tree();

    println!("Original tree");
    // Print tree with node
    // print!("{:#?}", tree);
    print!("{}", tree);
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

    // Look for a right node 'right_node' to replace it with 'new_str'
    let matcher = |kind| kind == SyntaxKind::NODE_ATTRPATH_VALUE;
    let matcher_str = |kind| kind == SyntaxKind::NODE_STRING;
    let tree = tree.syntax().clone_for_update();
    let right_node = tree
        .first_child().unwrap()
        .first_child().unwrap()
        .last_child().unwrap()
        .last_child().unwrap()
        .first_child_by_kind(&matcher).unwrap()
        .next_sibling_by_kind(&matcher).unwrap()
        .first_child_by_kind(&matcher_str).unwrap();
    // rnix::edit::insert() needs position, it can be created like this
    // let position = rnix::edit::Position::last_child_of(&right_node);

    rnix::edit::replace(right_node, new_str.clone_for_update());
    println!("{}", tree);
}
