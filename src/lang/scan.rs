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
	Semicolon,
	Slash,
	Star,
	Mod,

	Plus,
	PlusPlus,
	Minus,
	MinusMinus,
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

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

struct Scanner<'src> {
	source: Enumerate<Chars<'src>>,
	tokens: Vec<Token>,
}

impl<'src> Scanner<'src> {
	fn peek(&self, terminal: char) -> bool {
		let mut copy = self.source.clone();
		let Some((_, peek)) = copy.next() else {
			return false;
		};
		return peek == terminal
	}

	fn number(&mut self, init: char) -> Token {
		let mut tok_str = TokenStr::from(init);
		while let Some((_, terminal)) = self.source.next() {
			if !is_number(&terminal) {
				break;
			}
			tok_str.push(terminal);
		}
		Token::Number(tok_str)
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
	let mut scanner = Scanner { source: input.chars().enumerate(), tokens: Vec::new() };

	loop {
		let Some((i, terminal)) = scanner.source.next() else {
			scanner.tokens.push(Token::End);
			break;
		};

		if is_number(&terminal) {
		}
		match terminal  {
			'*' => scanner.tokens.push(Token::Star),
			'+' => (),

			' ' | '\n' => (),

			_   => return Err(Error::Terminal),
		}
	}

	println!("{:?}", scanner.tokens);

	Ok(scanner.tokens)
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
			];
			let file_toks = match do_file("add") {
				Ok(ts) => ts,
				Err(e) => return assert_eq!(true, false),
			};
			assert_eq!(file_toks, correct_toks)
		}
}
