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

fn is_number(terminal: &char) -> bool {
	match terminal {
		'0'..'9' | '.' => return true,
		_              => return false,
	}
}

fn is_identifier(terminal: &char) -> bool {
	match terminal {
		'a' .. 'z' |
		'A' .. 'Z' |
		'0' .. '9' |
		'_' => return true,
		_   => return false,
		
	}
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
		let Some((_, peek_term)) = self.source.next() else {
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
			if !is_number(&terminal) {
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
		Token::String(tok_str)
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
		let Some((_, terminal)) = scanner.source.next() else {
			tokens.push(Token::End);
			break;
		};

		if is_number(&terminal) {
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
			
		/* multi-terminal tokens */
			'=' |  /*  ==     */
			'!' |  /*  !=     */
			'*' |  /*  *=     */
			'-' |  /*  -=, -- */
			'/' |  /*  /=     */
			'&' |  /*  &&     */
			'+'    /*  ++, += */
			=> tokens.push(multi(&mut scanner, terminal)),

			' ' | '\n' => (),

			_   => return Err(Error::Terminal),
		}
	}

	Ok(tokens)
}

fn multi(scanner: &mut Scanner, terminal: char) -> Token {
	match terminal {
		'=' => {
			if scanner.match_char('=') {
				return Token::EqualEqual;
			}
			return Token::Equal;
		},
		'!' => {
			if scanner.match_char('=') {
				return Token::BangEqual;
			}
			return Token::Bang;
		},
		'*' => {
			if scanner.match_char('=') {
				return Token::StarEqual;
			}
			return Token::Star;
		},
		'-' => {
			if scanner.match_char('=') {
				return Token::MinusEqual;
			}
			return Token::Minus;
		},
		'/' => {
			if scanner.match_char('=') {
				return Token::SlashEqual;
			}
			return Token::Slash;
		},
		'&' => {
			if scanner.match_char('&') {
				return Token::And;
			}
		},
		'+' => {
			if scanner.match_char('=') {
				return Token::EqualEqual;
			}
			return Token::Equal;
		},
		_   => (),
	}
	
	return scanner.identifier(terminal);
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
}
