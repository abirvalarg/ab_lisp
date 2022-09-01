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

	// #[regex(r#""([^"]|\\.)*""#)]
	#[regex(r#""([^\\"]|\\[nrt\\0'"]|\\x[0-9a-f][0-9a-f])*""#)]
	String,

	#[token("(")]
	GroupStart,

	#[token(")")]
	GroupEnd,

	#[token("'(")]
	ListStart,

	#[error]
	#[regex("\\s", logos::skip)]
	#[regex(";.*\n", logos::skip)]
	Error
}
