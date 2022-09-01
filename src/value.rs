use std::{rc::Rc, cell::RefCell, fmt::{self, Debug}, f64::NAN, collections::HashMap};

use function::Function;
use list::List;
use number::Number;

use crate::{state::State, error::Error, location::Location};

use self::function::NativeFunction;

pub mod function;
pub mod list;
pub mod number;

#[derive(Clone)]
pub enum Value {
	Atom(String),
	Number(Number),
	String(String),
	List(Rc<List>),
	Function(Rc<Function>),
	Object(Rc<HashMap<String, Rc<RefCell<Value>>>>)
}

impl Value {
	pub fn nil() -> Self {
		Value::List(Rc::new(List::Nil))
	}

	pub fn native_function(func: NativeFunction) -> Self {
		Value::Function(Rc::new(Function::native(func)))
	}

	pub fn var(self) -> Rc<RefCell<Self>> {
		Rc::new(RefCell::new(self))
	}

	#[allow(dead_code)]
	pub fn list(data: &[Value]) -> Self {
		Value::List(List::from_slice(data))
	}

	pub fn to_number(&self) -> Number {
		match self {
			Value::Number(num) => num.clone(),
			_ => Number::Float(NAN)
		}
	}

	pub fn to_string(&self) -> String {
		match self {
			Value::String(val) => val.clone(),
			_ => format!("{self:?}")
		}
	}

	pub fn call(&self, abl: &mut State, args: Rc<List>, loc: &Location) -> Result<Value, Error> {
		use function::FunctionVal;
		match self {
			Value::Function(func) => {
				abl.push_scope(func.captures.clone());
				let res = match &func.val {
					FunctionVal::Native(func) => func(abl, &args.collect()[..]),
					FunctionVal::Lang { actions, args: arg_names } => {
						let mut args = args;
						for name in arg_names {
							abl.set_local(name, args.head().unwrap_or(&Value::nil()).clone());
							args = args.tail();
						}
						abl.execute(actions)
					}
				};
				abl.pop_scope();
				res
			}
			Value::Object(object) if !args.is_nil() => {
				let method = args.head().unwrap();
				if let Value::Atom(method) = method {
					match object.get(method) {
						Some(method) => {
							method.borrow().call(abl, args.tail(), loc)
						}
						None => Err(Error::new_at(crate::error::ErrorKind::NotAFunction, loc.clone()))
					}
				} else {
					Err(Error::new_at(crate::error::ErrorKind::BadIndex, loc.clone()))
				}
			}
			_ => Err(Error::new_at(crate::error::ErrorKind::NotAFunction, loc.clone()))
		}
	}
}

impl Into<bool> for Value {
	fn into(self) -> bool {
		match self {
			Value::List(list) => !list.is_nil(),
			Value::Number(num) => match num {
				Number::Int(x) => x != 0,
				Number::Float(x) =>  x != 0.
			},
			_ => true
		}
	}
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(arg0) => f.debug_tuple("Atom").field(arg0).finish(),
			Self::Number(Number::Int(num)) => write!(f, "{num}"),
			Self::Number(Number::Float(num)) => write!(f, "{num}"),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::List(arg0) => write!(f, "{:?}", arg0.collect()),
            Self::Function(arg0) => f.debug_tuple("Function").field(arg0).finish(),
            Self::Object(arg0) => f.debug_tuple("Object").field(arg0).finish(),
        }
    }
}
