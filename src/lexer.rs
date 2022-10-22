use std::str;
use crate::error::TokenizeError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenKind {
  Integer(usize),
  Float(f64),
  Plus,
  Minus,
  Divide,
  Multiply,
  Open,
  Close
}

impl From<usize> for TokenKind {
  fn from(other: usize) -> Self {
    TokenKind::Integer(other)
  }
}

impl From<f64> for TokenKind {
  fn from(other: f64) -> Self {
    TokenKind::Float(other)
  }
}

pub fn take_while<F: FnMut(char) -> bool>(data: &str, mut predicate: F) -> Result<(&str, usize), TokenizeError> {
  let mut index = 0;
  for char in data.chars() {
    if !predicate(char) { break };
    index += char.len_utf8();
  }
  if index == 0 { Err(TokenizeError::NoMatches) }
  else { Ok((&data[..index], index)) }
}

pub fn tokenize_number(data: &str) -> Result<(TokenKind, usize), TokenizeError> {
  let mut dot = false;

  let (value, read) = take_while(data, |char| {
    if char.is_digit(10) { true }
    else if char == '.' {
      if !dot { dot = true; true }
      else { false }
    }
    else { false }
  })?;

  if dot {
    let n: f64 = value.parse().expect(format!("Failed to parse float: {}", value).as_str());
    Ok((TokenKind::Float(n), read))
  } else {
    let n: usize = value.parse().expect(format!("Failed to parse float: {}", value).as_str());
    Ok((TokenKind::Integer(n), read))
  }
}

pub fn tokenize_single_token(data: &str) -> Result<(TokenKind, usize), TokenizeError> {
  let value = match data.chars().next() {
    Some(value) => value,
    None => return Err(TokenizeError::UnexpectedEOF)
  };
  let (kind, size) = match value {
    '+' => (TokenKind::Plus, 1),
    '-' => (TokenKind::Minus, 1),
    '*' => (TokenKind::Multiply, 1),
    '/' => (TokenKind::Divide, 1),
    '(' => (TokenKind::Open, 1),
    ')' => (TokenKind::Close, 1),
    '0'..='9' => tokenize_number(data)?,
    _ => return Err(TokenizeError::UnknownChar(value.to_string()))
  };
  Ok((kind, size))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Span {
  pub start: usize,
  pub end: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
  pub span: Span,
  pub kind: TokenKind,
}

impl Token {
  pub fn new<K: Into<TokenKind>>(span: Span, kind: K) -> Token {
    let kind = kind.into();
    Token { span, kind }
  }
}

impl<T> From<T> for Token
  where T: Into<TokenKind> {
  fn from(other: T) -> Token {
    Token::new(Span { start: 0, end: 0 }, other)
  }
}

struct Tokenizer<'a> {
  index: usize,
  remaining: &'a  str
}

impl<'a> Tokenizer<'a> {
  fn new(src: &str) -> Tokenizer {
    Tokenizer {
      index: 0,
      remaining: src
    }
  }

  fn next_token(&mut self) -> Result<Option<Token>, TokenizeError> {
    self.skip();

    if self.remaining.is_empty() { Ok(None) }
    else {
      let start =   self.index;
      let kind = self._next_token().expect("Couldn't read the next token");
      let end = self.index;
      Ok(Some(Token {
        span: Span { start, end },
        kind
      }))
    }
  }

  fn skip(&mut self) {
    self.chomp(skip_whitespaces(self.remaining));
  }

  fn chomp(&mut self, num_bytes: usize) {
    self.remaining = &self.remaining[num_bytes..];
    self.index += num_bytes;
  }

  fn _next_token(&mut self) -> Result<TokenKind, TokenizeError> {
    let (tok, bytes_read) = tokenize_single_token(self.remaining)?;
    self.chomp(bytes_read);
    Ok(tok)
  }
}

pub fn skip_whitespaces(data: &str) -> usize {
  match take_while(data, |char| char.is_whitespace()) {
    Ok((_, skipped)) => skipped,
    _ => 0
  }
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, TokenizeError> {
  let mut tokenizer = Tokenizer::new(src);
  let mut tokens = Vec::new();

  while let Some(token) = tokenizer.next_token()? {
    tokens.push(token);
  }

  Ok(tokens)
}

#[macro_export]
macro_rules! token {
    ($thing:tt) => {
      {
        #[allow(unused_imports)]
        use $crate::lexer::TokenKind::*;
        $crate::lexer::Token::from($thing)
      }
    };
}
