pub mod lexer;
pub mod parser;
mod parser_helper;
// mod main;

use std::fs;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn new_tree(file: &str) -> parser::Node {
    let mut s = fs::read_to_string(file).expect("Unable to read file").chars().collect();
    let mut tokens: Vec<lexer::LexToken> = Vec::new();
    let mut x = lexer::LexToken::ML_COMMENT(String::from("hi"));
    while x != lexer::LexToken::EOF {
        x = lexer::read_next(&mut s);
        tokens.push(x.clone());
    }
    // println!("Tokens: {:?}", tokens);
    let mut _file = parser::File{tokens: tokens, index: 0};
    parser::parse_toplevel(&mut _file)
}

