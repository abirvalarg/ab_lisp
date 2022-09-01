use std::{rc::Rc, cell::RefCell, fmt::{self, Debug}, f64::NAN};

use function::Function;
use list::List;
use number::Number;

use self::function::NativeFunction;

pub mod function;
pub mod list;
pub mod number;

#[derive(Clone)]
pub enum Value {
	Atom(String),
	Number(Number),
	List(Rc<List>),
	Function(Rc<Function>)
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
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(arg0) => f.debug_tuple("Atom").field(arg0).finish(),
			Self::Number(Number::Int(num)) => write!(f, "{num}"),
			Self::Number(Number::Float(num)) => write!(f, "{num}"),
            Self::List(arg0) => write!(f, "{:?}", arg0.collect()),
            Self::Function(arg0) => f.debug_tuple("Function").field(arg0).finish(),
        }
    }
}
