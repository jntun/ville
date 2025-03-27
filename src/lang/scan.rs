//
// Created by Justin Tunheim on 3/21/25
//

use std::iter::Enumerate;
use std::str::Chars;
use crate::lang;

#[derive(Debug)]
pub enum Error {
	File,
	Terminal,
	EndOfFile,
}

pub type TokenStr = String;

#[derive(Debug, PartialEq)]
pub enum Token {
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	LeftBracket,
	RightBracket,
	Comma,
	Dot,
	Colon,
	Semicolon,

	Slash,
	SlashEqual,
	Star,
	StarEqual,
	Mod,
	Plus,
	PlusPlus,
	PlusEqual,
	Minus,
	MinusMinus,
	MinusEqual,
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,
	And,

	Identifier(TokenStr),
	String(TokenStr),
	Number(TokenStr),

	End,
}

fn is_identifier(terminal: &char) -> bool {
	if terminal.is_alphabetic() || terminal.is_digit(10) {
		return true
	}
	false
}

struct Scanner<'src> {
	source: Enumerate<Chars<'src>>,
}

impl<'src> Scanner<'src> {
	fn peek(&self, terminal: char) -> bool {
		let mut copy = self.source.clone();
		let Some((_, peek)) = copy.next() else {
			return false;
		};
		return peek == terminal
	}

	fn match_char(&mut self, terminal: char) -> bool {
		let Some((i, peek_term)) = self.source.next() else {
			return false;
		};
		if peek_term != terminal {
			return false; 
		}
		true
	}

	fn number(&mut self, init: char) -> Token {
		let mut tok_str = TokenStr::from(init);
		while let Some((_, terminal)) = self.source.clone().peekable().peek() {
			if !terminal.is_digit(10) {
				break;
			}
			tok_str.push(self.source.next().unwrap().1);
		}
		Token::Number(tok_str)
	}

	fn identifier(&mut self, init: char) -> Token {
		let mut tok_str = TokenStr::from(init);
		while let Some((_, terminal)) = self.source.clone().peekable().peek() {
			if !is_identifier(&terminal) {
				break;
			}
			tok_str.push(self.source.next().unwrap().1);
		}
		Token::Identifier(tok_str)
	}
}

pub fn file(path: &String) -> Result<Vec<Token>, Error> {
	let src = match std::fs::read_to_string(path) {
		Ok(s)  => s,
		Err(_) => return Err(Error::File), // TODO: match on error and return specificity
	};
	
	source(src)
}

pub fn source(input: String) -> Result<Vec<Token>, Error> {
	let mut tokens  = Vec::new();
	let mut scanner = Scanner { source: input.chars().enumerate() };

	loop {
		let Some((i, terminal)) = scanner.source.next() else {
			tokens.push(Token::End);
			break;
		};

		if terminal.is_numeric() {
			tokens.push(scanner.number(terminal));
			continue;
		}
		match terminal  {
			'}' => tokens.push(Token::RightBrace),
			'{' => tokens.push(Token::LeftBrace),
			']' => tokens.push(Token::RightBracket),
			'[' => tokens.push(Token::LeftBracket),
			')' => tokens.push(Token::RightParen),
			'(' => tokens.push(Token::LeftParen),
			'%' => tokens.push(Token::Mod),
			';' => tokens.push(Token::Semicolon),
			':' => tokens.push(Token::Colon),
			'=' => tokens.push(Token::Equal),
			
			' ' | '\n' => (),

			_   => tokens.push(multi(&mut scanner, terminal)?),
		}
	}

	Ok(tokens)
}

fn multi(scanner: &mut Scanner, terminal: char) -> Result<Token, Error> {
	match terminal {
		'=' => {
			if scanner.match_char('=') {
				return Ok(Token::EqualEqual);
			}
			return Ok(Token::Equal);
		},
		'!' => {
			if scanner.match_char('=') {
				return Ok(Token::BangEqual);
			}
			return Ok(Token::Bang);
		},
		'*' => {
			if scanner.match_char('=') {
				return Ok(Token::StarEqual);
			}
			return Ok(Token::Star);
		},
		'-' => {
			if scanner.match_char('=') {
				return Ok(Token::MinusEqual);
			}
			return Ok(Token::Minus);
		},
		'/' => {
			if scanner.match_char('=') {
				return Ok(Token::SlashEqual);
			}
			return Ok(Token::Slash);
		},
		'&' => {
			if scanner.match_char('&') {
				return Ok(Token::And);
			}
		},
		'+' => {
			if scanner.match_char('=') {
				return Ok(Token::PlusEqual);
			}
			return Ok(Token::Plus);
		},
		_   => (),
	}
	
	if terminal.is_alphabetic() {
		return Ok(scanner.identifier(terminal));
	}
	Err(Error::Terminal)
}

#[cfg(test)]
mod tests {
		use super::*;

		fn do_file(filename: &str) -> Result<Vec<Token>, Error> {
			let mut path = String::from("tests/");
			path.push_str(filename);
			path.push_str(".");
			path.push_str(lang::Extension);

			file(&path)
		}

		#[test]
		fn test_add() {
			let correct_toks = vec![
				Token::Number(String::from("13")),
				Token::Star,
				Token::Number(String::from("5")),
				Token::Semicolon,
				Token::End,
			];
			let file_toks = match do_file("add") {
				Ok(ts) => ts,
				Err(e) => return assert_eq!(true, false),
			};
			assert_eq!(file_toks, correct_toks)
		}

		#[test]
		fn test_minus() {
			let correct_toks = vec![
				Token::Number(TokenStr::from("24")),
				Token::Minus,
				Token::Number(TokenStr::from("12")),
				Token::Semicolon,
				Token::End,
			];
			let file_toks = match do_file("minus") {
				Ok(ts) => ts,
				Err(e) => return assert_eq!(true, false),
			};
			assert_eq!(file_toks, correct_toks)
		}

		#[test]
		fn test_plus() {
			let correct_toks = vec![
				Token::Number(TokenStr::from("78")),
				Token::Plus,
				Token::Number(TokenStr::from("12")),
				Token::Semicolon,
				Token::Number(TokenStr::from("23")),
				Token::PlusEqual,
				Token::Number(TokenStr::from("98")),
				Token::Semicolon,
				Token::End,
			];
			let file_toks = match do_file("plus") {
				Ok(ts) => ts,
				Err(e) => return assert_eq!(true, false),
			};
			assert_eq!(file_toks, correct_toks)
		}

		#[test]
		fn test_star() {
			let correct_toks = vec![
				Token::Number(TokenStr::from("19")),
				Token::Star,
				Token::Number(TokenStr::from("73")),
				Token::Semicolon,
				Token::Number(TokenStr::from("38")),
				Token::StarEqual,
				Token::Number(TokenStr::from("27")),
				Token::Semicolon,
				Token::End,
			];
			let file_toks = match do_file("star") {
				Ok(ts) => ts,
				Err(e) => return assert_eq!(true, false),
			};
			assert_eq!(file_toks, correct_toks)
		}

		#[test]
		fn test_slash() {
			let correct_toks = vec![
				Token::Number(TokenStr::from("81")),
				Token::Slash,
				Token::Number(TokenStr::from("398")),
				Token::Semicolon,
				Token::Identifier(TokenStr::from("thing")),
				Token::Equal,
				Token::Number(TokenStr::from("64")),
				Token::Semicolon,
				Token::Identifier(TokenStr::from("thing")),
				Token::SlashEqual,
				Token::Number(TokenStr::from("18")),
				Token::Semicolon,
				Token::End
			];
			let file_toks = match do_file("slash") {
				Ok(ts) => ts,
				Err(e) => return assert_eq!(true, false),
			};
			assert_eq!(file_toks, correct_toks)
		}
}
