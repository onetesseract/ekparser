use std::usize;

use crate::lexer;
use crate::parser_helper;

#[derive(Debug)]
#[derive(Clone)]
pub enum Node {
    Prog(Box<Vec<Node>>),
    // ID then type
    Dec(lexer::LexToken, lexer::LexToken),
    // name, args, return type, body
    FnDef(Box<Node>, Box<Vec<Node>>, lexer::LexToken, Box<Node>),
    // Condition, then, else
    If(Box<Node>, Box<Node>, Box<Node>),
    // left, op, right
    Binary(Box<Node>, usize, Box<Node>),
    // for calls?
    ID(String), // todo: add resources (namespace, name)
    // Name, args
    Call(Box<Node>, Box<Vec<Node>>),
    Literal(Literal),
    // other, messy stuff
    Null, //TODO: remove

}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Literal {
    Num(f64),
    Bool(bool),
    Undef,
    None,
}

#[derive(Clone)]
pub struct File {
    pub tokens: Vec<lexer::LexToken>,
    pub index: usize,
}

const OPS: [&str; 14] = ["=", "||", "&&", /* now all 7 */ "<", ">", "<=", ">=", "==", "!=", /* all 10 */ "+", "-", /* 20s */ "*", "/", "%"];
const OP_STRENGTH: [usize; 14] = [1, 2, 3, 7, 7, 7, 7, 7, 7, 10, 10, 20, 20, 20];

fn parse_bool(f: &mut File) -> Node {
    Node::Literal(Literal::Bool(f.tokens[f.index] == lexer::LexToken::ID(String::from("true"))))
}
fn parse_dec(f: &mut File) -> Node {
    if !parser_helper::check_discriminant(&f.tokens[f.index], &lexer::LexToken::ID(String::new())) {
        println!("Unexpected {:?}, expected ID", f.tokens[f.index]);
        panic!();
    }
    if f.tokens[f.index+1] != lexer::LexToken::PUNC(String::from(":")) {
        println!("Unexpected {:?}, expected :", f.tokens[f.index]);
        panic!();
    }
    if !parser_helper::check_discriminant(&f.tokens[f.index], &lexer::LexToken::ID(String::new())) {
        println!("Unexpected {:?}, expected ID", f.tokens[f.index]);
        panic!();
    }
    let x = Node::Dec(f.tokens[f.index].clone(), f.tokens[f.index+2].clone());
    f.index += 3;
    x
}

fn maybe_dec(f: &mut File) -> bool {
    parser_helper::check_discriminant(&f.tokens[f.index], &lexer::LexToken::ID(String::new())) && f.tokens[f.index+1] == lexer::LexToken::PUNC(String::from(":"))
}

fn parse_if(f: &mut File) -> Node {
    parser_helper::skip(lexer::LexToken::ID(String::from("if")), f);
    // aaaa somehow parse conditions
    let x = parse_expression(f);
    let y = parse_prog(f);
    let z;
    if f.tokens[f.index] == lexer::LexToken::ID(String::from("else")) {
        f.index += 1;
        z = parse_prog(f);
    } else { z = Node::Null }
    return Node::If(Box::new(x), Box::new(y), Box::new(z));

}

fn maybe_fndef(f: &mut File, n: Node) -> Node {
    if !matches!(n, Node::Call(..)) {
        return n;
    }
    if let Node::Call(name, args) = n.clone() {
            if f.tokens[f.index] == lexer::LexToken::PUNC(String::from(":")) {
                f.index += 1;
                if let lexer::LexToken::ID(_) = f.tokens[f.index] {
                    let typ = f.tokens[f.index].clone();
                    f.index += 1;
                    if lexer::LexToken::PUNC(String::from("{")) != f.tokens[f.index] {
                        println!("Expected code block ({{)");
                        panic!();
                    }
                    return Node::FnDef(name, args, typ, Box::new(parse_prog(f)));
                } else {
                    println!("Expected a type!");
                    panic!();
                }
            } else if f.tokens[f.index] == lexer::LexToken::PUNC(String::from("{")) {
                return Node::FnDef(name, args, lexer::LexToken::FN_NULL_TYPE, Box::new(parse_prog(f)));
            } else {
                return n
            }
    } else {
        return n;
    }
}

fn maybe_call(f: &mut File, n: Node) -> Node {
    if f.tokens.len() < f.index + 2 {
        return n;
    }
    if f.tokens[f.index] == lexer::LexToken::PUNC(String::from("(")) {
        return parse_call(f, n);
    }
    n
}

fn parse_call(f: &mut File, name: Node) -> Node {
    let y = parser_helper::delimited(lexer::LexToken::PUNC(String::from("(")), lexer::LexToken::PUNC(String::from(")")), lexer::LexToken::PUNC(String::from(",")), f);
    return Node::Call(Box::new(name), Box::new(y));
    
}

fn maybe_binary(f: &mut File, left: Node, my_prec: usize) -> Node {
    let x = f.tokens[f.index].clone();
    f.index += 1;
    if let lexer::LexToken::OP(y) = x {
        let y: &str = &y;
        let a= OPS.iter().position(|&b| b == y ).expect("op not found");
        let his_prec = OP_STRENGTH[a];
        if my_prec < his_prec {
            let z = parse_atom(f);
            return maybe_binary(f, Node::Binary(Box::new(left), a, Box::new(z)), my_prec)
        }
    }
    f.index -= 1;
    return left;
}

fn pa_helper(f: &mut File) -> Node {
    if f.tokens[f.index] == lexer::LexToken::PUNC(String::from("(")) {
        f.index += 1;
        let ex = parse_expression(f);
        if f.tokens[f.index] != lexer::LexToken::PUNC(String::from(")")) {
            panic!("Expected )");
        }
        f.index += 1;
        return ex;
    }
    if f.tokens[f.index] == lexer::LexToken::PUNC(String::from("{")) {
        return parse_prog(f);
    }
    if f.tokens[f.index] == lexer::LexToken::ID(String::from("if")) {
        return parse_if(f);
    }
    if maybe_dec(f) {
        return parse_dec(f);
    }
    if f.tokens[f.index] == lexer::LexToken::ID(String::from("true")) || f.tokens[f.index] == lexer::LexToken::ID(String::from("false")) {
        return parse_bool(f);
    }
    if let lexer::LexToken::NUMBER(x) = f.tokens[f.index] {
        f.index+=1;
        return Node::Literal(Literal::Num(x));
    }
    if let lexer::LexToken::ID(x) = f.tokens[f.index].clone() {
        f.index+=1;
        return Node::ID(x)
    }
    panic!("Unexpected {:?}", f.tokens[f.index]);
}
fn parse_atom(f: &mut File) -> Node {
    let x = pa_helper(f);
    return maybe_call(f, x);
}

pub fn parse_expression(f: &mut File) -> Node {
    let y = parse_atom(f);
    let x = maybe_binary(f, y, 0);
    let z = maybe_call(f, x);
    return maybe_fndef(f, z);
}
fn parse_prog(f: &mut File) -> Node {
    if f.tokens[f.index] != lexer::LexToken::PUNC(String::from("{")) {
        return Node::Prog(Box::new(vec![parse_expression(f)]));
    }
    f.index += 1;
    let mut x: Vec<Node> = Vec::new();
    while f.tokens[f.index] != lexer::LexToken::PUNC(String::from("}")) && f.tokens[f.index] != lexer::LexToken::EOF {
        x.push(parse_expression(f));
        parser_helper::skip(lexer::LexToken::PUNC(String::from(";")),f);
    }
    parser_helper::skip(lexer::LexToken::PUNC(String::from("}")), f);
    Node::Prog(Box::new(x))
}

pub fn parse_toplevel(f: &mut File) -> Node {
    let mut x: Vec<Node> = Vec::new();
    while f.tokens[f.index] != lexer::LexToken::EOF {
        let z = parse_expression(f);
        x.push(z);
        parser_helper::skip(lexer::LexToken::PUNC(String::from(";")),f);
    }
    Node::Prog(Box::new(x))
}