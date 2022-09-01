use logos::Logos;

#[derive(Logos, Debug)]
pub enum Token {
	#[regex("[a-zA-z_+*/=<>!][a-zA-Z0-9_+*/=<>!-]*")]
	#[token("-")]
	Ident,

	#[regex("'[a-zA-Z0-9_-]+")]
	Atom,

	#[regex(r#"-?\d+"#)]
	Int,

	#[regex(r#"-?\d+\.\d*"#)]
	Float,

	#[token("(")]
	GroupStart,

	#[token(")")]
	GroupEnd,

	#[token("'(")]
	ListStart,

	// #[token("()")]
	// Nil,

	#[error]
	#[regex("\\s", logos::skip)]
	#[regex(";.*\n", logos::skip)]
	Error
}
