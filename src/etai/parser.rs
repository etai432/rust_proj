use crate::etai::ast::{Stmt, Program, Expr, BinaryExpr, NumericLiteral, Identifier};
use crate::etai::lexer::{tokenize, Token, TokenType};

struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn not_eof(&self) -> bool {
        self.tokens[0].token_type != TokenType::EOF
    }

    fn parse_stmt(&self) -> Box<dyn Stmt> {
        //skip to parse_expr
        return self.parse_expr();
    }

    fn parse_expr(&self) -> Box<dyn Expr> {

    }

    pub fn produceAST(&mut self, source_code: String) -> Program {
        self.tokens = tokenize(source_code);
        let program = Program::new();
        while self.not_eof() { //parse until the end of the file
            program.body.push(self.parse_stmt());
        }


        return program;
    }
}