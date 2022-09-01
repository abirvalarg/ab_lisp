use logos::{Logos, Lexer};

use crate::{action::{Action, ActionVal}, error::{Error, ErrorKind}, location::{self, Location}, value::{Value, number::Number}};

use self::token::Token;

mod token;

pub fn parse(source: location::Source, src: &str) -> Result<Vec<Action>, Error> {
	let mut lex = token::Token::lexer(src);
	let res = parse_rec(&source, &mut lex)?;
	if lex.next().is_none() {
		Ok(res)
	} else {
		Err(Error::new_at(ErrorKind::Syntax, res[res.len() - 1].location.clone()))
	}
}

fn parse_rec(source: &location::Source, lex: &mut Lexer<Token>) -> Result<Vec<Action>, Error> {
	let mut res = Vec::new();
	while let Some(token) = lex.next() {
		use token::Token::*;
		let location = Location {
			span: lex.span(),
			source: source.clone()
		};
		let val = match token {
			Ident => {
				let val = lex.slice();
				ActionVal::Ident(val.into())
			}
			Atom => {
				let val = &lex.slice()[1..];
				ActionVal::Literal(Value::Atom(val.into()))
			}
			Int => {
				let val = lex.slice().parse().unwrap();
				ActionVal::Literal(Value::Number(Number::Int(val)))
			}
			Float => {
				let val = lex.slice().parse().unwrap();
				ActionVal::Literal(Value::Number(Number::Float(val)))
			}
			String => {
				let val = enquote::unquote(lex.slice()).unwrap();
				ActionVal::Literal(Value::String(val))
			}
			GroupStart => {
				let content = parse_rec(source, lex)?;
				ActionVal::Group {
					content, quoted: false
				}
			}
			ListStart => {
				let content = parse_rec(source, lex)?;
				ActionVal::Group {
					content, quoted: true
				}
			}
			GroupEnd => {
				return Ok(res);
			}
			Error => {
				return Err(crate::error::Error::new_at(ErrorKind::Syntax, location));
			}
		};
		res.push(Action { location, val });
	}
	Ok(res)
}
