use logos::Span;

#[derive(Debug, Clone)]
pub struct Location {
	pub source: Source,
	pub span: Span
}

#[derive(Debug, Clone)]
pub enum Source {
	File(String)
}
