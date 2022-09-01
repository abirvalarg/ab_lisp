use std::{collections::{HashMap, LinkedList}, rc::Rc, cell::RefCell};

use crate::{prelude, value::{Value, function::Function, list::List}, action::{Action, ActionVal}, error::{Error, ErrorKind}};

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
		self.globals.insert("put-str".into(), Value::native_function(prelude::put_str).var());
		self.globals.insert("+".into(), Value::native_function(prelude::add).var());
		self.globals.insert("-".into(), Value::native_function(prelude::sub).var());
		self.globals.insert("*".into(), Value::native_function(prelude::mul).var());
		self.globals.insert("/".into(), Value::native_function(prelude::div).var());
		self.globals.insert("sqrt".into(), Value::native_function(prelude::sqrt).var());
		self.globals.insert(">=".into(), Value::native_function(prelude::ge).var());
	}

	pub fn get_var(&mut self, name: &str) -> Rc<RefCell<Value>> {
		match self.get_local(name) {
			Some(res) => res,
			None => match self.globals.get(name) {
				Some(var) => var.clone(),
				None => {
					self.globals.insert(name.into(), Value::nil().var());
					self.globals.get(name).unwrap().clone()
				} 
			}
		}
	}

	pub fn get_local(&mut self, name: &str) -> Option<Rc<RefCell<Value>>> {
		for scope in &self.scope {
			if let Some(var) = scope.get(name) {
				return Some(var.clone());
			}
		}
		None
	}

	pub fn set_local(&mut self, name: &str, value: Value) {
		let value = value.var();
		match self.scope.front_mut() {
			Some(scope) => scope.insert(name.into(), value),
			None => self.globals.insert(name.into(), value)
		};
	}

	pub(crate) fn push_scope(&mut self, scope: HashMap<String, Rc<RefCell<Value>>>) {
		self.scope.push_front(scope);
	}

	pub(crate) fn pop_scope(&mut self) {
		self.scope.pop_front();
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
						ActionVal::Ident(action) if action == "do" => self.execute(&content[1..]),
						ActionVal::Ident(action) if action == "let" => self.process_let_content(&content[1..]),
						ActionVal::Ident(action) if action == "set" => self.process_set_content(&content[1..]),
						ActionVal::Ident(action) if action == "if" => {
							if content.len() == 4 {
								let cond = self.eval(&content[1])?;
								if cond.into() {
									self.eval(&content[2])
								} else {
									self.eval(&content[3])
								}
							} else {
								Err(Error::new_at(ErrorKind::Syntax, content[0].location.clone()))
							}
						}
						ActionVal::Ident(action) if action == "function" => {
							if content.len() >= 4 {
								let name = if let ActionVal::Ident(name) = &content[1].val {
									name
								} else {
									return Err(Error::new_at(ErrorKind::Syntax, content[1].location.clone()));
								};

								let args = if let ActionVal::Group { content, .. } = &content[2].val {
									content
								} else {
									return Err(Error::new_at(ErrorKind::Syntax, content[2].location.clone()));
								};

								let func = Self::create_function(&args[..], &content[3..])?;
								let func = Value::Function(Rc::new(func));
								self.set_local(name, func.clone());
								Ok(func)
							} else {
								Err(Error::new_at(ErrorKind::Syntax, content[0].location.clone()))
							}
						}
						ActionVal::Ident(action) if action == "funcap" => {
							if content.len() >= 5 {
								let name = if let ActionVal::Ident(name) = &content[1].val {
									name
								} else {
									return Err(Error::new_at(ErrorKind::Syntax, content[1].location.clone()));
								};

								let args = if let ActionVal::Group { content, .. } = &content[2].val {
									content
								} else {
									return Err(Error::new_at(ErrorKind::Syntax, content[2].location.clone()));
								};

								let captures = if let ActionVal::Group { content, .. } = &content[3].val {
									content
								} else {
									return Err(Error::new_at(ErrorKind::Syntax, content[3].location.clone()));
								};

								let mut func = Self::create_function(&args[..], &content[4..])?;
								for cap in captures {
									if let ActionVal::Ident(name) = &cap.val {
										func.captures.insert(name.clone(), self.get_var(name));
									}
								}

								let func = Value::Function(Rc::new(func));
								self.set_local(name, func.clone());
								Ok(func)
							} else {
								Err(Error::new_at(ErrorKind::Syntax, content[0].location.clone()))
							}
						}
						ActionVal::Ident(action) if action == "object" => {
							let mut object = HashMap::new();
							for item in &content[1..] {
								if let ActionVal::Ident(name) = &item.val {
									if let Value::Function(method) = &*self.get_var(name).borrow() {
										object.insert(name.clone(), method.clone());
									}
								}
							}
							Ok(Value::Object(Rc::new(object)))
						}
						_ => {
							let data = self.eval_list(&content[..])?;
							let func = data.head().unwrap();
							let args = data.tail();
							func.call(self, args, &content[0].location)
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

	fn create_function(raw_args: &[Action], actions: &[Action]) -> Result<Function, Error> {
		let mut args = Vec::with_capacity(raw_args.len());
		for arg in raw_args {
			if let ActionVal::Ident(name) = &arg.val {
				args.push(name.clone());
			} else {
				return Err(Error::new_at(ErrorKind::Syntax, arg.location.clone()));
			}
		}
		let func = Function::lang(actions, args);
		Ok(func)
	}
}
