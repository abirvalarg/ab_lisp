use crate::{state::State, value::{Value, number::Number}, error::Error};

pub fn debug(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	for val in args {
		println!("{val:?}");
	}
	Ok(Value::nil())
}

pub fn put_str(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	for arg in args {
		print!("{}", arg.to_string());
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
	if args.len() == 1 {
		Ok(Value::Number(Number::Int(0) - args[0].to_number()))
	} else if args.len() > 0 {
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

pub fn sqrt(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	if args.len() > 0 {
		let x = match args[0].to_number() {
			Number::Int(x) => x as f64,
			Number::Float(x) => x
		};
		Ok(Value::Number(Number::Float(x.sqrt())))
	} else {
		Ok(Value::Number(Number::Float(1.)))
	}
}

pub fn ge(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	match args.len() {
		0 => Ok(Value::Number(0.into())),
		1 => Ok(Value::Number( (if args[0].to_number() >= 0.into() { 1 } else { 0 }).into() )),
		_ => Ok(Value::Number( (if args[0].to_number() >= args[1].to_number() { 1 } else { 0 }).into() ))
	}
}
