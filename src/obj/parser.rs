use obj::object::{
    ObjectSet, Object, 
    Vertex, TextureVertex, NormalVertex,
    Element
};
use lexer::Lexer;
use std::iter;
use std::collections::HashMap;
use obj::vertex_parser::VertexParser;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    line_number: usize,
    message: String,
}

impl ParseError {
    fn new(line_number: usize, message: String) -> ParseError {
        ParseError {
            line_number: line_number,
            message: message,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum ParserIndex {
    Start,
    VertexParser,
}

enum SubParser {
    Vertex(VertexParser),
}

pub struct ParserState<'a> {
    line_number: usize,
    lexer: iter::Peekable<Lexer<'a>>,
    state_index: ParserIndex,
    state_table: HashMap<ParserIndex, SubParser>,
}

impl<'a> ParserState<'a> {
    pub fn new(input: &'a str) -> ParserState<'a> {
        let mut state_table = HashMap::new();
        state_table.insert(ParserIndex::VertexParser, SubParser::Vertex(VertexParser::new()));

        ParserState {
            line_number: 1,
            lexer: Lexer::new(input).peekable(),
            state_index: ParserIndex::Start,
            state_table: state_table,
        }
    }

    pub fn peek(&mut self) -> Option<String> {
        self.lexer.peek().map(|s| s.clone())
    }

    pub fn next(&mut self) -> Option<String> {
        let token = self.lexer.next();
        match token {
            Some(ref val) => {
                if val == "\n" {
                    self.line_number += 1;
                }
            },
            None => {},
        }

        token
    }

    pub fn advance(&mut self) {
        self.next();
    }

    pub fn error<T>(&mut self, err: String) -> Result<T, ParseError> {
        Err(ParseError::new(self.line_number, err))
    }

    pub fn parse_string(&mut self) -> Result<String, ParseError> {
        match self.next() {
            Some(st) => Ok(st),
            None => self.error(format!("Expected string but got `end of line`."))
        }
    }

    pub fn parse_statement(&mut self, tag: &str) -> Result<String, ParseError> {
        let st = try!(self.parse_string());
        match st == tag {
            true => Ok(st),
            false => self.error(format!("Expected `{}` statement but got: {}.", tag, st))
        }
    }

    pub fn parse_f32(&mut self) -> Result<f32, ParseError> {
        let st = try!(self.parse_string());
        match st.parse() {
            Ok(val) => Ok(val),
            Err(_) => self.error(
                format!("Expected `f32` but got `{}`.", st)
            ),
        }
    }

    pub fn parse_u32(&mut self) -> Result<u32, ParseError> {
        let st = try!(self.parse_string());
        match st.parse() {
            Ok(val) => Ok(val),
            Err(_) => self.error(format!("Expected integer but got `{}`.", st)),
        }
    }

    pub fn try_once<P, T>(&mut self, parser: P) -> Option<T> where P: FnOnce(&str) -> Option<T> {
        match self.peek() {
            Some(st) => {
                parser(&st).map(|got| { self.advance(); got })
            },
            None => None,
        }
    }
}

struct StartParser {}
impl StartParser {
    fn new() -> StartParser { StartParser {} }

    fn parse(&self, state: &mut ParserState) { }

}

