use std::rc::Rc;

use super::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum List {
	Nil,
	Item {
		value: Value,
		next: Rc<List>
	}
}

impl List {
	pub fn new() -> Rc<Self> {
		Rc::new(List::Nil)
	}

	#[allow(dead_code)]
	pub fn from_slice(data: &[Value]) -> Rc<Self> {
		if data.len() == 0 {
			Rc::new(List::Nil)
		} else {
			let value = data[0].clone();
			let next = List::from_slice(&data[1..]);
			Rc::new(List::Item { value, next })
		}
	}

	pub fn push_front(self: Rc<Self>, value: Value) -> Rc<Self> {
		Rc::new(List::Item {
			value,
			next: self
		})
	}

	pub fn head(&self) -> Option<&Value> {
		match self {
			List::Nil => None,
			List::Item { value, .. } => Some(value)
		}
	}

	pub fn tail(&self) -> Rc<Self> {
		match self {
			List::Nil => List::new(),
			List::Item { next, .. } => next.clone()
		}
	}

	#[allow(dead_code)]
	pub fn is_nil(&self) -> bool {
		match self {
			List::Nil => true,
			_ => false
		}
	}

	pub fn len(&self) -> usize {
		match self {
			List::Nil => 0,
			List::Item { next, .. } => next.len() + 1
		}
	}

	pub fn collect(&self) -> Vec<Value> {
		let mut res = Vec::with_capacity(self.len());
		self.collect_rec(&mut res);
		res
	}

	fn collect_rec(&self, res: &mut Vec<Value>) {
		match self {
			List::Nil => (),
			List::Item { value, next } => {
				res.push(value.clone());
				next.collect_rec(res);
			}
		}
	}
}
