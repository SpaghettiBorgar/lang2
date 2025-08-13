use std::cmp::min;

use crate::lexing_rules::{MatchPattern, RULES};
use crate::token::{Token, TokenKind};

#[derive(PartialEq, Debug)]
pub enum CharKind {
	ChWhite,
	ChAlpha,
	ChNum,
	ChSpecial,
	ChOther,
	ChNull,
}

#[derive(PartialEq, Debug)]
enum StateFlag {
	StInit,    //Initial, non-accepting
	StParsing, //non-accepting
	StAccept,  //currently accepting
	StDiscard, //final non-accepting
	StFinal,   //completed lexeme
	StFail,    //error
}

use CharKind::*;
use StateFlag::*;
use TokenKind::*;

#[inline]
pub fn char_kind(c: char) -> CharKind {
	match c {
		c if c.is_whitespace() => ChWhite,
		c if c.is_alphabetic() => ChAlpha,
		c if c.is_digit(10) => ChNum,
		c if "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".contains(c) => ChSpecial,
		'\0' => ChNull,
		_ => ChOther,
	}
}
struct MatchState {
	token_kind: TokenKind,
	pattern: &'static MatchPattern,
	frag: usize,
	flag: StateFlag,
	quant: i8,
}

impl MatchState {
	fn new(token_kind: TokenKind, pattern: &'static MatchPattern) -> Self {
		Self {
			token_kind: token_kind,
			pattern: &pattern,
			frag: 0,
			flag: StInit,
			quant: i8::MAX,
		}
	}
}
impl std::fmt::Debug for MatchState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{:?}:{:?}({}*{})",
			self.token_kind, self.flag, self.frag, self.quant
		)
	}
}

fn iterate(c: char, state: &mut MatchState) {
	let rule = &state.pattern[state.frag];
	let patlen = state.pattern.len();
	if state.flag == StDiscard || state.flag == StFail {
		state.flag = StFail;
		panic!();
	}
	if state.quant == i8::MAX {
		state.quant = rule.quantity;
	}
	let is_matching = (rule.matcher)(c);
	let is_optional = state.quant < 0;
	if is_matching {
		state.quant -= 1;
		if state.quant == 0 {
			state.frag += 1;
			state.quant = i8::MAX;
		}
		if state.frag >= patlen {
			state.flag = StFinal;
			return;
		}
		let accepting = !(state.quant > 0 && state.quant != i8::MAX
			|| state.pattern[min(state.frag + 1, patlen - 1)..patlen - 1]
				.iter()
				.any(|frag| frag.quantity >= 0));
		state.flag = match accepting {
			false => StParsing,
			true => StAccept,
		};
	} else {
		state.frag += 1;
		state.quant = i8::MAX;
		if !is_optional {
			state.flag = StDiscard;
		} else if state.frag < patlen {
			iterate(c, state);
		} else {
			state.flag = StDiscard;
		}
	}
}

pub fn lex(text: &mut String) -> Vec<Token> {
	let mut tokens = Vec::new();
	text.push('\0');
	let mut chars = text.chars().peekable();

	'tokens: loop {
		let mut states = RULES.map(|(tk_kind, pattern_ref)| MatchState::new(tk_kind, pattern_ref));
		let mut buf = String::new();
		let state = 'chars: loop {
			if let Some(c) = chars.peek() {
				println!("{} {:?}", (*c as u8), states);
				for s in states.iter_mut() {
					if s.flag == StDiscard {
						continue;
					}
					iterate(*c, s);
					match s.flag {
						StInit => panic!(),
						StParsing => (),
						StAccept => (),
						StDiscard => (),
						StFinal => break 'chars s,
						StFail => panic!(),
					}
				}
			} else {
				if !(buf.is_empty() || buf.bytes().nth(0) == Some(b'\0')) {
					panic!("non-empty buffer at EOF");
				}
				tokens.push(Token {
					kind: TkEOF,
					val: String::new(),
				});
				break 'tokens;
			}
			buf.push(chars.next().unwrap());
		};
		tokens.push(Token {
			kind: state.token_kind,
			val: buf,
		});
	}
	return tokens;
}

pub fn _lex(text: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut chars = text.chars().peekable();
	while chars.peek().is_some() {
		let mut c = chars.peek().unwrap().clone();
		let c_kind = char_kind(c);
		let mut buf = String::new();
		match c_kind {
			ChWhite => {
				chars.next();
			}
			ChAlpha => {
				while matches!(char_kind(chars.peek().unwrap().clone()), ChAlpha | ChNum) {
					c = chars.next().unwrap();
					buf.push(c);
				}
				buf.shrink_to_fit();
				tokens.push(Token {
					kind: TkIdent,
					val: buf,
				});
			}
			ChNum => {
				while matches!(char_kind(chars.peek().unwrap().clone()), ChNum) {
					c = chars.next().unwrap();
					buf.push(c);
				}
				tokens.push(Token {
					kind: TkNum,
					val: buf,
				});
			}
			ChSpecial => {
				c = chars.next().unwrap();
				buf.push(c);
				let t_kind = if ";{}".contains(c) {
					TkSep
				} else if "=+-*/".contains(c) {
					TkOp
				} else {
					panic!("Unknown special char");
				};
				tokens.push(Token {
					kind: t_kind,
					val: buf,
				});
			}
			ChOther | ChNull => {
				panic!("Unknown Character '{}'", c);
			}
		};
	}
	tokens.push(Token {
		kind: TkEOF,
		val: String::new(),
	});
	tokens
}
