use std::fs::File;
use std::io::{BufRead, Write, BufReader};
use std::env;

#[derive(PartialEq, Clone, Debug, Eq)]
pub enum TokenType {
    EOF,
    Illegal,
    Identifier,
    IntegerLiteral,
    CharLiteral,
    Operator,
    Array,
    Begin,
    Const,
    Do,
    Else,
    End,
    Func,
    If,
    In,
    Let,
    Of,
    Proc,
    Record,
    Then,
    Type,
    Var,
    While,
    Period,
    Colon,
    Semicolon,
    Comma,
    Equals,
    Tilde,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Assign,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub row: usize,
    pub col: usize,
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, row: usize, col: usize) -> Self {
        Token {
            token_type,
            lexeme,
            row,
            col,
        }
    }
}

#[derive(Debug)]
pub enum ASTNode {
    Let(Box<ASTNode>, Box<ASTNode>),
    Const(String, Box<ASTNode>),
    Var(String, String),
    Func(String, Vec<ASTNode>, String, Box<ASTNode>),
    Proc(String, Vec<ASTNode>, Box<ASTNode>),
    Type(String, Box<ASTNode>),
    Assign(Vec<String>, Box<ASTNode>),
    If(Box<ASTNode>, crate::TokenType, Box<ASTNode>, crate::TokenType, Box<ASTNode>),
    While(Box<ASTNode>, Box<ASTNode>),
    Call(String, Vec<ASTNode>),
    Expression(Box<ASTNode>),
    Identifier(String),
    Number(i64),
    Char(char),
    Operator(String, Box<ASTNode>, Box<ASTNode>),
    Declaration(Vec<ASTNode>),
    Command(Vec<ASTNode>),
}
impl ASTNode {
    fn to_custom_string(&self) -> String {
        match self {
            ASTNode::Let(declarations, command) => format!(
                "let(\n   {},\n   {}\n)",
                declarations.to_custom_string(),
                command.to_custom_string()
            ),
            ASTNode::Const(name, value) => format!(
                "const(\n   name(\"{}\"),\n   value(\n      {}\n   )\n)",
                name,
                value.to_custom_string()
            ),
            ASTNode::Var(name, type_name) => format!(
                "var(name(\"{}\"),typeName(\"{}\"))",
                name, type_name
            ),
            ASTNode::Func(name, params, return_type, body) => format!(
                "func(\n   name(\"{}\"),\n   params([{}]),\n   type(typeName(\"{}\")),\n   result(\n      {}\n   )\n)",
                name,
                params.iter().map(|p| p.to_custom_string()).collect::<Vec<_>>().join(","),
                return_type,
                body.to_custom_string()
            ),
            ASTNode::Assign(names, expr) => format!(
                "assign({:?}, {})",
                names,
                expr.to_custom_string()
            ),
            ASTNode::If(cond, _, then_branch, _, else_branch) => format!(
                "ifCmd(\n   cond({}),\n   then(\n      {}\n   ),\n   else({})\n)",
                cond.to_custom_string(),
                then_branch.to_custom_string(),
                else_branch.to_custom_string()
            ),
            ASTNode::Call(name, params) => format!(
                "call(\"{}\",params([{}]))",
                name,
                params.iter().map(|p| p.to_custom_string()).collect::<Vec<_>>().join(",")
            ),
            ASTNode::Operator(op, left, right) => format!(
                "op(\n   {},\n   {},\n   {}\n)",
                op,
                left.to_custom_string(),
                right.to_custom_string()
            ),
            ASTNode::Number(num) => format!("num({})", num),
            ASTNode::Char(c) => format!("char({})", *c as u8),
            ASTNode::Identifier(name) => format!("ref([\"{}\"])", name),
            ASTNode::Declaration(declarations) => format!(
                "declaration(\n   [{}]\n)",
                declarations.iter().map(|d| d.to_custom_string()).collect::<Vec<_>>().join(",\n   ")
            ),
            ASTNode::Command(commands) => format!(
                "command(\n   [{}]\n)",
                commands.iter().map(|c| c.to_custom_string()).collect::<Vec<_>>().join(",\n   ")
            ),
            ASTNode::Expression(expr) => expr.to_custom_string(),
            ASTNode::Proc(_, _, _) | ASTNode::Type(_, _) | ASTNode::While(_, _) => {
                "Unsupported node".to_string()
            }
        }
    }
}

pub struct SyntaxParser {
    pub current: Token,
    pub tokens: Vec<Token>,
    pub index: usize,
}
impl SyntaxParser {
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        let current = tokens.get(0).cloned().unwrap_or(Token::new(TokenType::EOF, "".to_string(), 0, 0));
        SyntaxParser {
            current,
            tokens,
            index: 0,
        }
    }

    fn advance(&mut self) {
        if self.index < self.tokens.len() - 1 {
            self.index += 1;
            self.current = self.tokens[self.index].clone();
        }
    }

    fn expect_token(&mut self, expected: TokenType) -> Result<(), String> {
        if self.current.token_type == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, but found {:?} at row {}, col {}",
                expected, self.current.token_type, self.current.row, self.current.col
            ))
        }
    }

    pub fn parse_program(&mut self) -> Result<ASTNode, String> {
        self.process_commands()
    }

    fn process_commands(&mut self) -> Result<ASTNode, String> {
        let mut commands = vec![self.parse_single_command()?];

        while self.current.token_type == TokenType::Semicolon {
            self.advance();
            commands.push(self.parse_single_command()?);
        }

        if commands.len() == 1 {
            Ok(commands.pop().unwrap())
        } else {
            Ok(ASTNode::Command(commands))
        }
    }

    fn parse_single_command(&mut self) -> Result<ASTNode, String> {
        match self.current.token_type {
            TokenType::Let => {
                self.advance();
                let declarations = self.process_declarations()?;
                self.expect_token(TokenType::In)?;
                let commands = self.process_commands()?;
                Ok(ASTNode::Let(Box::new(declarations), Box::new(commands)))
            }
            TokenType::Const => {
                self.advance();
                let name = self.capture_identifier()?;
                self.expect_token(TokenType::Tilde)?;
                let expression = self.parse_expression()?;
                Ok(ASTNode::Const(name, Box::new(expression)))
            }
            TokenType::Var => {
                self.advance();
                let variable_name = self.capture_identifier()?;
                self.expect_token(TokenType::Colon)?;
                let type_identifier = self.capture_identifier()?;
                Ok(ASTNode::Var(variable_name, type_identifier))
            }
            TokenType::Func => {
                self.advance();
                let func_name = self.capture_identifier()?;
                self.expect_token(TokenType::LeftParen)?;
                let parameters = self.get_formal_parameters()?;
                self.expect_token(TokenType::RightParen)?;
                self.expect_token(TokenType::Colon)?;
                let return_type = self.capture_identifier()?;
                self.expect_token(TokenType::Tilde)?;
                let func_body = self.parse_expression()?;
                Ok(ASTNode::Func(func_name, parameters, return_type, Box::new(func_body)))
            }
            TokenType::If => {
                self.advance();
                let condition = self.parse_expression()?;
                self.expect_token(TokenType::Then)?;
                let then_block = self.process_commands()?;
                self.expect_token(TokenType::Else)?;
                let else_block = self.process_commands()?;
                Ok(ASTNode::If(
                    Box::new(condition),
                    TokenType::Then,
                    Box::new(then_block),
                    TokenType::Else,
                    Box::new(else_block),
                ))
            }
            TokenType::Begin => {
                self.advance();
                let inner_commands = self.process_commands()?;
                self.expect_token(TokenType::End)?;
                Ok(inner_commands)
            }
            TokenType::Identifier => {
                let id = self.capture_identifier()?;
                if self.current.token_type == TokenType::Assign {
                    self.advance();
                    let expr = self.parse_expression()?;
                    Ok(ASTNode::Assign(vec![id], Box::new(expr)))
                } else if self.current.token_type == TokenType::LeftParen {
                    self.advance();
                    let args = self.get_actual_parameters()?;
                    self.expect_token(TokenType::RightParen)?;
                    Ok(ASTNode::Call(id, args))
                } else {
                    Err(format!(
                        "Unexpected token: {:?} at row {}, col {}",
                        self.current.token_type, self.current.row, self.current.col
                    ))
                }
            }
            _ => Err(format!(
                "Unexpected command token: {:?} at row {}, col {}",
                self.current.token_type, self.current.row, self.current.col
            )),
        }
    }

    fn get_formal_parameters(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut params = Vec::new();
        if self.current.token_type != TokenType::RightParen {
            params.push(self.parse_formal_parameter()?);
            while self.current.token_type == TokenType::Comma {
                self.advance();
                params.push(self.parse_formal_parameter()?);
            }
        }
        Ok(params)
    }

    fn parse_formal_parameter(&mut self) -> Result<ASTNode, String> {
        let mut is_var = false;
        if self.current.token_type == TokenType::Var {
            is_var = true;
            self.advance(); // Avanza si se encuentra 'var'
        }
        let name = self.capture_identifier()?;
        self.expect_token(TokenType::Colon)?;
        let type_name = self.capture_identifier()?;
        
        if is_var {
            Ok(ASTNode::Var(name, type_name))
        } else {
            // Manejar como parámetro no variable
            Ok(ASTNode::Var(name, type_name))
        }
    }

    fn get_actual_parameters(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut params = Vec::new();
        if self.current.token_type != TokenType::RightParen {
            params.push(self.parse_expression()?);
            while self.current.token_type == TokenType::Comma {
                self.advance();
                params.push(self.parse_expression()?);
            }
        }
        Ok(params)
    }

    fn capture_identifier(&mut self) -> Result<String, String> {
        if self.current.token_type == TokenType::Identifier {
            let id_name = self.current.lexeme.clone();
            self.advance();
            Ok(id_name)
        } else {
            Err(format!(
                "Expected identifier, found {:?} at row {}, col {}",
                self.current.token_type, self.current.row, self.current.col
            ))
        }
    }

    fn process_declarations(&mut self) -> Result<ASTNode, String> {
        let mut decls = vec![self.parse_single_declaration()?];
        while self.current.token_type == TokenType::Semicolon {
            self.advance();
            decls.push(self.parse_single_declaration()?);
        }
        if decls.len() == 1 {
            Ok(decls.pop().unwrap())
        } else {
            Ok(ASTNode::Declaration(decls))
        }
    }

    fn parse_single_declaration(&mut self) -> Result<ASTNode, String> {
        match self.current.token_type {
            TokenType::Const => {
                self.advance();
                let name = self.capture_identifier()?;
                self.expect_token(TokenType::Tilde)?;
                let expr = self.parse_expression()?;
                Ok(ASTNode::Const(name, Box::new(expr)))
            }
            TokenType::Var => {
                self.advance();
                let var_name = self.capture_identifier()?;
                self.expect_token(TokenType::Colon)?;
                let var_type = self.capture_identifier()?;
                Ok(ASTNode::Var(var_name, var_type))
            }
            TokenType::Func => {
                self.advance();
                let func_id = self.capture_identifier()?;
                self.expect_token(TokenType::LeftParen)?;
                let param_list = self.get_formal_parameters()?;
                self.expect_token(TokenType::RightParen)?;
                self.expect_token(TokenType::Colon)?;
                let ret_type = self.capture_identifier()?;
                self.expect_token(TokenType::Tilde)?;
                let func_body = self.parse_expression()?;
                Ok(ASTNode::Func(func_id, param_list, ret_type, Box::new(func_body)))
            }
            _ => Err(format!(
                "Unexpected declaration token: {:?} at row {}, col {}",
                self.current.token_type, self.current.row, self.current.col
            )),
        }
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let left = self.parse_primary_expression()?;
        self.parse_expression_prime(left)
    }

    fn parse_expression_prime(&mut self, left: ASTNode) -> Result<ASTNode, String> {
        if self.current.token_type == TokenType::Operator {
            let operator = self.current.lexeme.clone();
            self.advance();
            let right = self.parse_primary_expression()?;
            let expr = ASTNode::Operator(operator, Box::new(left), Box::new(right));
            self.parse_expression_prime(expr)
        } else {
            Ok(left)
        }
    }

    fn parse_primary_expression(&mut self) -> Result<ASTNode, String> {
        match self.current.token_type {
            TokenType::IntegerLiteral => {
                let num_value = self.current.lexeme.parse::<i64>().unwrap();
                self.advance();
                Ok(ASTNode::Number(num_value))
            }
            TokenType::CharLiteral => {
                let char_value = self.current.lexeme.chars().next().unwrap();
                self.advance();
                Ok(ASTNode::Char(char_value))
            }
            TokenType::Identifier => {
                let id = self.capture_identifier()?;
                if self.current.token_type == TokenType::LeftParen {
                    self.advance();
                    let param_list = self.get_actual_parameters()?;
                    self.expect_token(TokenType::RightParen)?;
                    Ok(ASTNode::Call(id, param_list))
                } else {
                    Ok(ASTNode::Identifier(id))
                }
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect_token(TokenType::RightParen)?;
                Ok(ASTNode::Expression(Box::new(expr)))
            }
            _ => Err(format!(
                "Unexpected primary expression token: {:?} at row {}, col {}",
                self.current.token_type, self.current.row, self.current.col
            )),
        }
    }
}

fn write_custom_ast_to_file(ast: &ASTNode, file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    writeln!(file, "{}", ast.to_custom_string())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: parse <input_file> [-o <output_file>]");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = if args.len() > 3 && args[2] == "-o" {
        &args[3]
    } else {
        "tree.out"
    };

    let file = File::open(input_file).expect("Unable to open input file");
    let reader = BufReader::new(file);

    let mut tokens = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 4 {
            let token_type_str = parts[0].trim_matches('{').trim();
            let lexeme = parts[1].trim().trim_matches('\'').to_string();
            let row: usize = parts[2].trim().parse().expect("Invalid row number");
            let col: usize = parts[3].trim_matches('}').trim().parse().expect("Invalid column number");
            let token_type = match token_type_str {
                "EOF" => TokenType::EOF,
                "Illegal" => TokenType::Illegal,
                "Identifier" => TokenType::Identifier,
                "IntegerLiteral" => TokenType::IntegerLiteral,
                "CharLiteral" => TokenType::CharLiteral,
                "Operator" => TokenType::Operator,
                "Array" => TokenType::Array,
                "Begin" => TokenType::Begin,
                "Const" => TokenType::Const,
                "Do" => TokenType::Do,
                "Else" => TokenType::Else,
                "End" => TokenType::End,
                "Func" => TokenType::Func,
                "If" => TokenType::If,
                "In" => TokenType::In,
                "Let" => TokenType::Let,
                "Of" => TokenType::Of,
                "Proc" => TokenType::Proc,
                "Record" => TokenType::Record,
                "Then" => TokenType::Then,
                "Type" => TokenType::Type,
                "Var" => TokenType::Var,
                "While" => TokenType::While,
                "Period" => TokenType::Period,
                "Colon" => TokenType::Colon,
                "Semicolon" => TokenType::Semicolon,
                "Comma" => TokenType::Comma,
                "Equals" => TokenType::Equals,
                "Tilde" => TokenType::Tilde,
                "LeftParen" => TokenType::LeftParen,
                "RightParen" => TokenType::RightParen,
                "LeftBracket" => TokenType::LeftBracket,
                "RightBracket" => TokenType::RightBracket,
                "LeftBrace" => TokenType::LeftBrace,
                "RightBrace" => TokenType::RightBrace,
                "Assign" => TokenType::Assign,
                _ => {
                    eprintln!("Invalid token type: {}", token_type_str);
                    std::process::exit(1);
                }
            };
            tokens.push(Token::new(token_type, lexeme, row, col));
        }
    }
    let mut parser = SyntaxParser::from_tokens(tokens);
    let ast = parser.parse_program();
    match ast {
        Ok(ast) => {
            let result = write_custom_ast_to_file(&ast, output_file);
            if let Err(e) = result {
                eprintln!("Error writing to output file: {}", e);
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Parse error: {}", err);
            std::process::exit(1);
        }
    }
}
