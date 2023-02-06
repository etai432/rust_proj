#![allow(dead_code)]
use std::{collections::HashMap, thread::panicking};

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TokenType {
    Number,
    Identifier,
    Equals,
    OpenParen, 
    CloseParen,
    BinaryOperator,
    Let,
    EOF, //end of file
}

fn token(value: String, token_type: TokenType) -> Token {
    Token {value: value, token_type: token_type}
}

fn isalpha(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn isint(ch: char) -> bool {
    ch.is_ascii_digit()
}

fn is_skipable(ch: char) -> bool {
    ch == ' ' || ch == '\n' || ch == '\t' || ch == '\r'
}

pub fn tokenize(source_code: String) -> Vec<Token> {
    //reserved keywords
    let keywords: HashMap<String, TokenType> = vec![
        ("let".to_string(), TokenType::Let)
        ].into_iter().collect::<HashMap<String, TokenType>>();
    let mut tokens: Vec<Token> = Vec::new();
    let mut src: Vec<char> = source_code.chars().collect();
    //build each token until end of file
    while src.len() > 0 {
        if src[0] == '(' {
            tokens.push(token(src.remove(0).to_string(), TokenType::OpenParen));
        } else if src[0] == ')' {
            tokens.push(token(src.remove(0).to_string(), TokenType::CloseParen))
        } else if src[0] == '+' || src[0] == '-' || src[0] == '*' || src[0] == '/' {
            tokens.push(token(src.remove(0).to_string(), TokenType::BinaryOperator))
        } else if src[0] == '=' {
            tokens.push(token(src.remove(0).to_string(), TokenType::Equals))
        } else { //handles multicharacter tokens
            if isint(src[0]) { //build number token
                let mut num = String::new();
                while src.len() > 0 && isint(src[0]) {
                    num.push(src.remove(0));
                }
                tokens.push(token(num, TokenType::Number))
            } else if isalpha(src[0]) {
                let mut ident = String::new();
                while src.len() > 0 && isalpha(src[0]) {
                    ident.push(src.remove(0));
                }
                let key = match keywords.get(&ident) {
                    Some(key) => *key,
                    None => TokenType::Identifier,
                };
                tokens.push(token(ident, key));
            } else if is_skipable(src[0]) {
                src.remove(0);
            } else {
                println!("unregocnized character: {}, ascii: {:?}", src[0], src[0] as u8);
                panic!();
            }
        }
    }
    tokens.push(token("EndOfFile".to_string(), TokenType::EOF));
    return tokens;
}


