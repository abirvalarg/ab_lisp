use crate::{state::State, value::Value, error::Error};

pub fn test(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	println!("test function with {} args", args.len());
	Ok(Value::nil())
}

pub fn debug(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	for val in args {
		println!("{val:?}");
	}
	Ok(Value::nil())
}
