use std::{fmt::{self, Debug}, collections::HashMap, rc::Rc, cell::RefCell};

use crate::{state::State, error::Error, action::Action};

use super::Value;

pub type NativeFunction = fn(abl: &mut State, args: &[Value]) -> Result<Value, Error>;

#[derive(Debug)]
pub struct Function {
	pub val: FunctionVal,
	pub captures: HashMap<String, Rc<RefCell<Value>>>
}

impl Function {
	pub fn native(func: NativeFunction) -> Self {
		Function {
			val: FunctionVal::Native(func),
			captures: HashMap::new(),
		}
	}

	pub fn lang(actions: &[Action], args: Vec<String>) -> Self {
		Function {
			val: FunctionVal::Lang {
				actions: actions.into(),
				args
			},
			captures: HashMap::new()
		}
	}
}

pub enum FunctionVal {
	Native(NativeFunction),
	Lang {
		actions: Vec<Action>,
		args: Vec<String>
	}
}

impl Debug for FunctionVal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			FunctionVal::Native(_) => write!(f, "<native function>"),
			&FunctionVal::Lang { .. } => write!(f, "<function>")
		}
	}
}
