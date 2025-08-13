use crate::lexer::*;
use crate::token::*;

use TokenKind::*;

trait LiteralMatching {
	fn literal_match(self, c: char) -> bool;
}

impl LiteralMatching for char {
	fn literal_match(self, c: char) -> bool {
		c == self
	}
}

impl LiteralMatching for &str {
	fn literal_match(self, c: char) -> bool {
		self.contains(c)
	}
}

fn literal_match<T: LiteralMatching>(c: char, lit: T) -> bool {
	<T as LiteralMatching>::literal_match(lit, c)
}

macro_rules! rule_expr {
	($c:ident ($($inner:tt)+)) => {
		(rule_expr!($c $($inner)+))
	};

	($c:ident not($($inner:tt)+)) => {
		!(rule_expr!($c $($inner)+))
	};

    ($c:ident $a:tt and $($b:tt)+) => {
        (rule_expr!($c $a) && rule_expr!($c $($b)+))
    };

    ($c:ident $a:tt or $($b:tt)+) => {
        (rule_expr!($c $a) || rule_expr!($c $($b)+))
    };

    ($c:ident $v:ident) => {
        char_kind($c) == CharKind::$v
    };

	($c:ident $lit:literal) => {{
		literal_match($c, $lit)
	}};

    ($c:ident #$func:ident) => {
        $func($c)
    };

	($($a:tt)*) => {
		panic!("illegal macro case")
	};
}

macro_rules! make_frag {
    ($quant:literal, $($expr:tt)*) => {
		PatternFrag {
        	matcher: |c: char| rule_expr!(c $($expr)*),
			quantity: $quant
		}
    };
}

fn any(_c: char) -> bool {
	true
}

pub struct PatternFrag {
	pub matcher: fn(char) -> bool,
	pub quantity: i8,
}
pub type MatchPattern = [PatternFrag];

pub const RULES: [(TokenKind, &MatchPattern); 6] = [
	(
		TkWhite,
		&[make_frag!(0, ChWhite), make_frag!(1, not(ChWhite))],
	),
	(
		TkNum,
		&[make_frag!(0, (ChNum)), make_frag!(1, not(ChNum or ChAlpha))],
	),
	(
		TkKeyword,
		&[
			make_frag!(1, 'l'),
			make_frag!(1, 'e'),
			make_frag!(1, 't'),
			make_frag!(1, not(ChAlpha or ChNum)),
		],
	),
	(
		TkIdent,
		&[
			make_frag!(1, ChAlpha or '_'),
			make_frag!(-1, ChAlpha or ChNum or '_'),
			make_frag!(1, not(ChAlpha or ChNum or '_')),
		],
	),
	(TkOp, &[make_frag!(1, '='), make_frag!(1, not(ChSpecial))]),
	(TkSep, &[make_frag!(1, ";{}"), make_frag!(1, #any)]),
];
