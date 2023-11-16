use atlas_core::{utils::span::{WithSpan, BytePos, Span}, interfaces::lexer::{Lexer, token::Token}};
use std::{fs::File, iter::Peekable, collections::HashMap, path::PathBuf};
use std::io::{BufRead, BufReader};
use Token::*;

/// Default Lexer and base one for the Atlas77 language
pub struct SimpleLexerV1 {
    file_path: PathBuf,
    current_pos: BytePos,
    it: Peekable<std::vec::IntoIter<char>>,
    keywords: HashMap<String, Token>,
}

impl Lexer for SimpleLexerV1 {
    fn with_text(&mut self, text: String) -> Result<(), std::io::Error> {
        self.it = text.chars().collect::<Vec<_>>().into_iter().peekable();
        let mut keywords = HashMap::new();
        keywords.insert("struct".to_owned(), KwStruct);
        keywords.insert("else".to_owned(), KwElse);
        keywords.insert("false".to_owned(), KwFalse);
        keywords.insert("List".to_owned(), KwList);
        keywords.insert("Map".to_owned(), KwMap);
        keywords.insert("if".to_owned(), KwIf);
        keywords.insert("return".to_owned(), KwReturn);
        keywords.insert("true".to_owned(), KwTrue);
        keywords.insert("let".to_owned(), KwLet);
        keywords.insert("char".to_owned(), KwChar);
        keywords.insert("f64".to_owned(), KwF64);
        keywords.insert("i64".to_owned(), KwI64);
        keywords.insert("string".to_owned(), KwString);
        keywords.insert("bool".to_owned(), KwBool);
        keywords.insert("enum".to_owned(), KwEnum);
        keywords.insert("do".to_owned(), KwDo);
        keywords.insert("end".to_owned(), KwEnd);
        keywords.insert("then".to_owned(), KwThen);

        self.keywords = keywords;

        Ok(())
    }
    fn with_file_path(&mut self, file_path: PathBuf) -> Result<(), std::io::Error> {
        let file = File::open(file_path.clone())?;
        let reader = BufReader::new(file);

        let mut source_code = Vec::new();
        for line in reader.lines() {
            source_code.extend(line?.chars());
            source_code.push('\n'); // Add newline character for line breaks
        }
        let mut keywords = HashMap::new();
        keywords.insert("struct".to_owned(), KwStruct);
        keywords.insert("else".to_owned(), KwElse);
        keywords.insert("false".to_owned(), KwFalse);
        keywords.insert("List".to_owned(), KwList);
        keywords.insert("Map".to_owned(), KwMap);
        keywords.insert("if".to_owned(), KwIf);
        keywords.insert("return".to_owned(), KwReturn);
        keywords.insert("true".to_owned(), KwTrue);
        keywords.insert("let".to_owned(), KwLet);
        keywords.insert("char".to_owned(), KwChar);
        keywords.insert("f64".to_owned(), KwF64);
        keywords.insert("i64".to_owned(), KwI64);
        keywords.insert("string".to_owned(), KwString);
        keywords.insert("bool".to_owned(), KwBool);
        keywords.insert("enum".to_owned(), KwEnum);
        keywords.insert("do".to_owned(), KwDo);
        keywords.insert("end".to_owned(), KwEnd);
        keywords.insert("then".to_owned(), KwThen);
        keywords.insert("match".to_owned(), KwMatch);
        keywords.insert("loop".to_owned(), KwLoop);
        keywords.insert("break".to_owned(), KwBreak);

        self.keywords = keywords;
        self.file_path = file_path;
        self.it = source_code.into_iter().peekable();

        Ok(())
    }

    fn tokenize(&mut self) -> Vec<WithSpan<Token>> {
        let mut tokens: Vec<WithSpan<Token>> = Vec::new();

        loop {
            let start_pos = self.current_pos;
            let ch = match self.next() {
                None => break,
                Some(c) => c,
            };
            
            if let Some(t_token) = self.match_t_token(ch) {
                tokens.push(WithSpan {
                    span: Span {
                        start: start_pos,
                        end: self.current_pos,
                    },
                    value: t_token,
                })
            }
        }

        tokens
    }
}

impl SimpleLexerV1 {
    /// Create a new empty `SimpleLexerV1`
    pub fn new() -> Self {
        SimpleLexerV1 {
            file_path: PathBuf::default(),
            it: " ".chars().collect::<Vec<_>>().into_iter().peekable(),
            current_pos: BytePos::default(),
            keywords: HashMap::new()
        }
    }

    fn next(&mut self) -> Option<char> {
        let next = self.it.next();
        if let Some(ch) = next {
            self.current_pos = self.current_pos.shift(ch);
        }
        next
    }

    fn peek(&mut self) -> Option<&char> {
        self.it.peek()
    }

    fn either(&mut self, to_match: char, matched: Token, unmatched: Token) -> Token {
        if self.consume_if(|c| c == to_match) {
            matched
        } else {
            unmatched
        }
    }

    fn consume_if<F>(&mut self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        if let Some(&ch) = self.it.peek() {
            if f(ch) {
                self.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn consume_if_next<F>(&mut self, f: F) -> bool 
    where
        F: Fn(char) -> bool
    {
        let mut it = self.it.clone();
        match it.next() {
            None => return false,
            _ => (),
        }

        if let Some(&ch) = it.peek() {
            if f(ch) {
                self.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn consume_while<F>(&mut self, f: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        let mut chars: Vec<char> = Vec::new();
        while let Some(&ch) = self.peek() {
            if f(ch) {
                self.next().unwrap();
                chars.push(ch);
            } else {
                break;
            }
        }
        chars
    }

    fn match_t_token(&mut self, ch: char) -> Option<Token> {
        use Token::*;
        match ch {
            '\n' | '\t' | ' ' | '\r' => {
                if !self.peek().is_none() {
                    let ch = self.next().unwrap();
                    self.match_t_token(ch)
                } else {
                    None
                }
            },
            '(' => Some(LParen),
            ')' => Some(RParen),
            '{' => Some(LBrace),
            '}' => Some(RBrace),
            '[' => Some(LBracket),
            ']' => Some(RBracket),
            '+' => Some(self.either('=', OpAssignAdd, OpAdd)),
            '-' => {
                if self.consume_if(|c| c == '>') {
                    Some(RArrow)
                } else {
                    Some(self.either('=', OpAssignSub, OpSub))
                }
            }
            '*' => Some(self.either('=', OpAssignMul, OpMul)),
            //TODO: Add support for multiline comments
            '/' => {
                if self.consume_if(|c| c == '/') {
                    self.consume_while(|c| c != '\n');
                    if !self.peek().is_none() {
                        let ch = self.next().unwrap();
                        self.match_t_token(ch)
                    } else {
                        None
                    }
                } else {
                    Some(self.either('=', OpAssignDiv, OpDiv))
                }
            },
            '\\' => {
                Some(BackSlash)
            }
            '%' => Some(self.either('=', OpAssignMod, OpMod)),
            '^' => Some(OpPow),
            '<' => {
                if self.consume_if(|c| c == '=') {
                    Some(OpLe)
                } else {
                    Some(self.either('-', LArrow, OpLt))
                }
            },
            '>' => Some(self.either('=', OpGe, OpGt)),
            '=' => {
                if self.consume_if(|ch| ch == '>') {
                    Some(FatArrow)
                } else {
                    Some(self.either('=', OpEq, OpAssign))
                }
            },
            '&' => {
                Some(self.either('&', OpAnd, Ampersand))
            },
            '|' => {
                Some(self.either('|', OpOr, Pipe))
            },
            '!' => {
                Some(self.either('=', OpNe, OpNot))
            },
            //Logical
            ':' => {
                Some(self.either(':', DoubleColon, Colon))
            },
            ';' => {
                Some(Semicolon)
            },
            ',' => {
                Some(Comma)
            },
            '.' => {
                Some(self.either('.', DoubleDot, Dot))
            },
            //Identifiers
            ch if ch.is_alphabetic() || ch == '_' => {
                Some(self.identifier(ch).unwrap())
            },
            x if x.is_numeric() => {
                Some(self.number(x).unwrap())
            },
            '"' => {
                let mut string = String::new();
                string.push_str(self.consume_while(|ch| ch != '"').iter().collect::<String>().as_ref());
                self.next().unwrap();
                Some(String_(string))
            },
            //TODO: Be able to use the escape character (backslash) in strings and chars
            '\'' => {
                let ch = self.next().unwrap();
                self.next().unwrap();
                Some(Char(ch))
            },
            '?' => Some(Question),

            c => Some(Unknown(c))
        }
    }

    fn identifier(&mut self, c: char) -> Option<Token> {
        let mut ident = String::new();
        ident.push(c);

        while let Some(&ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(self.next().unwrap());
            } else {
                break;
            }
        }

        if let Some(k) = self.keywords.get(&ident) {
            Some(k.clone())
        } else {
            Some(Token::Ident(ident))
        }        
    }

    fn number(&mut self, c: char) -> Option<Token> {
        let mut number = String::new();
        number.push(c);

        let num: String = self
            .consume_while(|a| a.is_numeric())
            .into_iter()
            .collect();
        number.push_str(&num);

        if self.peek() == Some(&'.') && self.consume_if_next(|c| c.is_numeric()) {
            number.push('.');

            let num: String = self
                .consume_while(|a| a.is_numeric())
                .into_iter()
                .collect();
            number.push_str(&num);

            Some(Token::Float(number.parse::<f64>().unwrap()))
        } else {
            Some(Token::Int(number.parse::<i64>().unwrap()))
        }
    }
    
}