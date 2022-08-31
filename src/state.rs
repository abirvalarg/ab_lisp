use std::{collections::{HashMap, LinkedList}, rc::Rc, cell::RefCell};

use crate::{prelude, value::{Value, function::{Function, FunctionVal}, list::List}, action::{Action, ActionVal}, error::{Error, ErrorKind}};

pub struct State {
	globals: HashMap<String, Rc<RefCell<Value>>>,
	scope: LinkedList<HashMap<String, Rc<RefCell<Value>>>>
}

impl State {
	pub fn new() -> Self {
		State {
			globals: HashMap::new(),
			scope: LinkedList::new()
		}
	}

	pub fn reg_prelude(&mut self) {
		let test = Value::Function(Rc::new(Function::native(prelude::test)));
		self.globals.insert("test".into(), test.var());

		let debug = Value::Function(Rc::new(Function::native(prelude::debug)));
		self.globals.insert("debug".into(), debug.var());
	}

	pub fn get_var(&mut self, name: &str) -> Rc<RefCell<Value>> {
		for scope in &self.scope {
			if let Some(var) = scope.get(name) {
				return var.clone();
			}
		}
		match self.globals.get(name) {
			Some(var) => var.clone(),
			None => {
				self.globals.insert(name.into(), Value::nil().var());
				self.globals.get(name).unwrap().clone()
			} 
		}
	}

	pub fn execute(&mut self, actions: &[Action]) -> Result<Value, Error> {
		let mut ret = Value::nil();
		for action in actions {
			ret = self.eval(action)?;
		}
		Ok(ret)
	}

	fn eval(&mut self, action: &Action) -> Result<Value, Error> {
		match &action.val {
			ActionVal::Literal(val) => Ok(val.clone()),
			ActionVal::Ident(name) => Ok(self.get_var(&name).borrow().clone()),
			ActionVal::Group { content, quoted: true } => {
				let content = self.eval_list(&content[..])?;
				Ok(Value::List(content))
			}
			ActionVal::Group { content, quoted: false } => {
				let data = self.eval_list(&content[..])?;
				if data.is_nil() {
					Ok(Value::List(data))
				} else {
					let func = data.head().unwrap();
					let args = data.tail().collect();
					self.scope.push_front(HashMap::new());
					let res = match func {
						Value::Function(func) => match &func.val {
							FunctionVal::Native(func) => func(self, &args[..])
						}
						_ => Err(Error::new_at(ErrorKind::NotAFunction, content[0].location.clone()))
					};
					self.scope.pop_front();
					res
				}
			}
		}
	}

	fn eval_list(&mut self, actions: &[Action]) -> Result<Rc<List>, Error> {
		if actions.len() == 0 {
			Ok(List::new())
		} else {
			let val = self.eval(&actions[0])?;
			let next = self.eval_list(&actions[1..])?;
			Ok(next.push_front(val))
		}
	}
}
