use crate::lexer;
use crate::parser;

use std::mem;

pub fn skip(token: lexer::LexToken, f: &mut parser::File) -> lexer::LexToken {
    if f.tokens[f.index] == token {
        f.index += 1;
        return f.tokens[f.index-1].clone();
    }
    else {
        println!("Unexpected {:?}, expected {:?}", f.tokens[f.index], token);
        panic!();
    }
}

pub fn check_discriminant(token: & lexer::LexToken, tk2: & lexer::LexToken) -> bool{
    mem::discriminant(token) == mem::discriminant(tk2)
}

pub fn delimited(start: lexer::LexToken, stop: lexer::LexToken, sep: lexer::LexToken, f: &mut parser::File) -> Vec<parser::Node> {
    skip(start, f);
    let mut x: Vec<parser::Node> = Vec::new();
    if f.tokens[f.index] == stop {
        skip(stop, f);
        return x;
    }
    while f.index < f.tokens.len() && f.tokens[f.index] != lexer::LexToken::EOF {  
        if f.tokens[f.index] == stop {
            skip(stop, f);
            break;
        }  
        x.push(parser::parse_expression(f));
        if f.tokens[f.index] == stop {
            skip(stop, f);
            break;
        }
        // skip(sep.clone(), f);
        if f.tokens[f.index] != sep {
            println!("Unexpected {:?}, expected {:?}", f.tokens[f.index], sep);
            panic!();
        } else { f.index += 1; }
        if f.tokens[f.index] == stop {
            skip(stop, f);
            break;
        }
    }
    return x;
}