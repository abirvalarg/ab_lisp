use std::{collections::{HashMap, LinkedList}, rc::Rc, cell::RefCell};

use crate::{prelude, value::{Value, function::FunctionVal, list::List}, action::{Action, ActionVal}, error::{Error, ErrorKind}};

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
		self.globals.insert("debug".into(), Value::native_function(prelude::debug).var());
		self.globals.insert("+".into(), Value::native_function(prelude::add).var());
		self.globals.insert("-".into(), Value::native_function(prelude::sub).var());
		self.globals.insert("*".into(), Value::native_function(prelude::mul).var());
		self.globals.insert("/".into(), Value::native_function(prelude::div).var());
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

	pub fn set_local(&mut self, name: &str, value: Value) {
		let value = value.var();
		match self.scope.front_mut() {
			Some(scope) => scope.insert(name.into(), value),
			None => self.globals.insert(name.into(), value)
		};
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
				if content.len() == 0 {
					Ok(Value::nil())
				} else {
					match &content[0].val {
						ActionVal::Ident(action) if action == "let" => self.process_let_content(&content[1..]),
						ActionVal::Ident(action) if action == "set" => self.process_set_content(&content[1..]),
						_ => {
							let data = self.eval_list(&content[..])?;
							let func = data.head().unwrap();
							let args = data.tail().collect();
							match func {
								Value::Function(func) => {
									self.scope.push_front(func.captures.clone());
									let res = match func.val {
										FunctionVal::Native(func) => func(self, &args[..])
									};
									self.scope.pop_front();
									res
								}
								_ => Err(Error::new_at(ErrorKind::NotAFunction, content[0].location.clone()))
							}
						}
					}
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

	fn process_let_content(&mut self, content: &[Action]) -> Result<Value, Error> {
		if content.len() > 0 {
			match &content[0].val {
				ActionVal::Ident(name) => {
					let value = if content.len() >= 2 {
						self.eval(&content[1])?
					} else {
						Value::nil()
					};
					self.set_local(name, value.clone());
					if content.len() > 2 {
						self.process_let_content(&content[2..])
					} else {
						Ok(value)
					}
				}
				_ => Err(Error::new_at(ErrorKind::Syntax, content[0].location.clone()))
			}
		} else {
			Ok(Value::nil())
		}
	}

	fn process_set_content(&mut self, content: &[Action]) -> Result<Value, Error> {
		if content.len() > 0 {
			match &content[0].val {
				ActionVal::Ident(name) => {
					let value = if content.len() >= 2 {
						self.eval(&content[1])?
					} else {
						Value::nil()
					};
					let var = self.get_var(name);
					*var.borrow_mut() = value.clone();
					if content.len() > 2 {
						self.process_let_content(&content[2..])
					} else {
						Ok(value)
					}
				}
				_ => Err(Error::new_at(ErrorKind::Syntax, content[0].location.clone()))
			}
		} else {
			Ok(Value::nil())
		}
	}
}
