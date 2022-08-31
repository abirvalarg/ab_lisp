use std::fmt::{self, Display};

use ariadne::{Label, Source, Span};

use crate::location::{Location, self};

#[derive(Debug)]
pub struct Error {
	location: Option<Location>,
	kind: ErrorKind
}

impl Error {
	pub fn new(kind: ErrorKind) -> Self {
		Error {
			location: None,
			kind
		}
	}

	pub fn new_at(kind: ErrorKind, location: Location) -> Self {
		Error {
			location: Some(location),
			kind
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use ErrorKind::*;
		match &self.kind {
			File(path) => write!(f, "Can't open file `{path}`"),
			Usage => write!(f, "Usage: ab_lisp <path>"),
			Syntax => {
				let location = self.location.as_ref().unwrap();
				let rep = ariadne::Report::build(ariadne::ReportKind::Error, (), location.span.start())
					.with_label(Label::new(location.span.clone()))
					.finish();
				let mut buf: Vec<u8> = Vec::new();
				let src = match &location.source {
					location::Source::File(path) => std::fs::read_to_string(path).unwrap()
				};
				rep.write(Source::from(src), &mut buf).unwrap();
				write!(f, "{}", String::from_utf8(buf).unwrap())
			}
			NotAFunction => match &self.location {
				Some(loc) => {
					let mut buf: Vec<u8> = Vec::new();
					let src = match &loc.source {
						location::Source::File(path) => std::fs::read_to_string(path).unwrap()
					};
					ariadne::Report::build(ariadne::ReportKind::Error, (), loc.span.start())
						.with_label(Label::new(loc.span.clone()).with_message("Attempt to call a non-function value"))
						.finish()
						.write(Source::from(src), &mut buf).unwrap();
					write!(f, "{}", String::from_utf8(buf).unwrap())
				}
				None => write!(f, "Attempt to call a non-function value")
			}
		}
	}
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum ErrorKind {
	File(String),
	Usage,
	Syntax,
	NotAFunction
}
