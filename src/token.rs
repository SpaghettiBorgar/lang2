use std::fmt::{self};

#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
	TkWhite,
	TkIdent,
	TkNum,
	TkKeyword,
	TkOp,
	TkSep,
	TkEOF,
}

#[derive(Clone)]
pub struct Token {
	pub kind: TokenKind,
	pub val: String,
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}({})", self.kind, self.val)
	}
}
impl fmt::Debug for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}({})", self.kind, self.val)
	}
}
