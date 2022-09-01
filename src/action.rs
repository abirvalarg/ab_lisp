use crate::{value::Value, location::Location};

#[derive(Debug, Clone)]
pub struct Action {
	pub val: ActionVal,
	pub location: Location
}

#[derive(Debug, Clone)]
pub enum ActionVal {
	Ident(String),
	Literal(Value),
	Group {
		content: Vec<Action>,
		quoted: bool
	}
}
