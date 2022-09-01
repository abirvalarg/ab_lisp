use crate::{state::State, value::{Value, number::Number}, error::Error};

pub fn debug(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	for val in args {
		println!("{val:?}");
	}
	Ok(Value::nil())
}

pub fn add(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	let mut sum = Number::Int(0);
	for arg in args {
		sum += arg.to_number();
	}
	Ok(Value::Number(sum))
}

pub fn sub(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	if args.len() > 0 {
		let mut res = args[0].to_number();
		for arg in &args[1..] {
			res -= arg.to_number();
		}
		Ok(Value::Number(res))
	} else {
		Ok(Value::Number(Number::Int(0)))
	}
}

pub fn mul(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	let mut prod = Number::Int(1);
	for arg in args {
		prod *= arg.to_number();
	}
	Ok(Value::Number(prod))
}

pub fn div(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	if args.len() > 0 {
		let mut res = args[0].to_number();
		for arg in &args[1..] {
			res /= arg.to_number();
		}
		Ok(Value::Number(res))
	} else {
		Ok(Value::Number(Number::Int(1)))
	}
}
