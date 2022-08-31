use std::{rc::Rc, cell::RefCell, fmt::{self, Debug}};

use function::Function;
use list::List;

pub mod function;
pub mod list;

#[derive(Clone)]
pub enum Value {
	Atom(String),
	List(Rc<List>),
	Function(Rc<Function>)
}

impl Value {
	pub fn nil() -> Self {
		Value::List(Rc::new(List::Nil))
	}

	pub fn var(self) -> Rc<RefCell<Self>> {
		Rc::new(RefCell::new(self))
	}

	#[allow(dead_code)]
	pub fn list(data: &[Value]) -> Self {
		Value::List(List::from_slice(data))
	}
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(arg0) => f.debug_tuple("Atom").field(arg0).finish(),
            Self::List(arg0) => write!(f, "{:?}", arg0.collect()),
            Self::Function(arg0) => f.debug_tuple("Function").field(arg0).finish(),
        }
    }
}
