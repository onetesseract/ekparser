const PUNCS: [&str; 11] = ["{", "}", "::", ":", ";", ".", ",", "(", "[", "]", ")"];
const OPS: [&str; 14] = ["=", "||", "&&", /* now all 7 */ "<", ">", "<=", ">=", "==", "!=", /* all 10 */ "+", "-", /* 20s */ "*", "/", "%"];

#[derive(Debug)]
#[derive(PartialEq)]
pub enum CharClass {
    ID_START,
    OP,
    WHITESPACE,
    LETTER,
    DIGIT,
    PUNC,
    UNKNOWN,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum LexToken {
    ID(String),
    NUMBER(f64),
    PUNC(String),
    OP(String),
    SL_COMMENT(String),
    ML_COMMENT(String),
    FN_NULL_TYPE,
    EOF,
    ERROR,
}

fn classify(c: char) -> CharClass {
    match c {
        'A'..='Z' | 'a'..='z' | '_'=> CharClass::LETTER,
        '0'..='9' => CharClass::DIGIT,
        '\n' | '\t' | '\r' | ' ' => CharClass::WHITESPACE,
        _ => {
            if "+-=/<>*%^&|".contains(c) {
                CharClass::OP
            } else if "(){}.,;:\"'".contains(c) {
                CharClass::PUNC
            } else {
                println!("Unknown {}", c);
                CharClass::UNKNOWN
            }
        }
    }
}
// should we ret now, or is there more?
fn check_op(s: &Vec<char>) -> bool {
    if s.len() < 1 { return true; }
    let mut string = String::new();
    string.push(s[0]);
    if s.len() < 2 { return true; }
    string.push(s[1]);
    if OPS.contains(&(&string as &str)) { return false }
    return true;
}

// should we ret now, or is there more?
fn check_punc(s: &Vec<char>) -> bool {
    if s.len() < 1 { return true; }
    let mut string = String::new();
    string.push(s[0]);
    if s.len() < 2 { return true; }
    string.push(s[1]);
    if PUNCS.contains(&(&string as &str)) { return false }
    return true;
}

pub fn read_next(s: &mut Vec<char>) -> LexToken {
    if s.len() == 0 {
        return LexToken::EOF;
    }
    let mut ret = String::new();
    let c = classify(s[0]);
    if c == CharClass::OP {
        let mut pnc = String::new();
        // let mut is_two = false;
        if !check_op(s) {
            pnc.push(s[0]);
            pnc.push(s[1]);
            s.remove(0);
            s.remove(0);
        } else {
            pnc.push(s[0]);
            s.remove(0);
        }
        return LexToken::OP(pnc);
    }
    if c == CharClass::PUNC {
        let mut pnc = String::new();
        // let mut is_two = false;
        if !check_punc(s) {
            pnc.push(s[0]);
            pnc.push(s[1]);
            s.remove(0);
            s.remove(0);
        } else {
            pnc.push(s[0]);
            s.remove(0);
        }
        return LexToken::PUNC(pnc);
    }
    if s[0] == '/' {
        if s.len() == 1 {}
        else if s[1] == '/' { // hit a sl comment
            s.remove(0);
            s.remove(0);
            let mut comment = String::new();
            while s.len() != 0 && s[0] != '\n' {
                comment.push(s[0]);
                s.remove(0);
            }
            return LexToken::SL_COMMENT(comment);
        } else if s[1] == '*' { // hit a multiline comment
            s.remove(0);
            s.remove(0);
            let mut comment = String::new();
            while s.len() > 1 {
                if s[0] == '*' && s[1] == '/' {
                    s.remove(0);
                    s.remove(0);
                    break;
                } else {
                    comment.push(s[0]);
                    s.remove(0);
                }
            }
            return LexToken::ML_COMMENT(comment);
        }
    }
    if classify(s[0]) == CharClass::LETTER {
        while s.len() != 0 && classify(s[0]) == CharClass::LETTER || classify(s[0]) == CharClass::DIGIT   {
            ret.push(s[0]);
            s.remove(0);
        }
    }
    while s.len() != 0 && classify(s[0]) == c  {
        ret.push(s[0]);
        s.remove(0);
    }
    if c == CharClass::DIGIT {
        if s.len() != 0 && s[0] == '.' {
            ret.push('.');
            s.remove(0);
            while s.len() != 0 && classify(s[0]) == c  {
                ret.push(s[0]);
                s.remove(0);
            }
        }
    }
    match c {
        CharClass::OP => LexToken::OP(ret),
        CharClass::DIGIT => LexToken::NUMBER(ret.parse().unwrap()),
        CharClass::LETTER => LexToken::ID(ret),
        CharClass::WHITESPACE => read_next(s),
        CharClass::PUNC => LexToken::ERROR, //never called
        _ => { println!("Error at {}", ret); LexToken::ERROR },
    }
}