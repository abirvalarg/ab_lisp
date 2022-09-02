use crate::{state::State, value::{Value, number::Number, list::List}, error::Error};

pub fn debug(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	for val in args {
		println!("{val:?}");
	}
	Ok(Value::nil())
}

pub fn print(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	for arg in args {
		print!("{}", arg.to_string());
	}
	Ok(Value::nil())
}

pub fn head(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	if args.len() == 0 {
		Ok(Value::nil())
	} else {
		match &args[0] {
			Value::List(list) => Ok(list.head().unwrap_or(&Value::nil()).clone()),
			_ => Ok(args[0].clone())
		}
	}
}

pub fn tail(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	if args.len() == 0 {
		Ok(Value::nil())
	} else {
		match &args[0] {
			Value::List(list) => Ok(Value::List(list.tail())),
			_ =>Ok(Value::nil())
		}
	}
}

pub fn cons(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	if args.len() == 0 {
		Ok(Value::nil())
	} else {
		let mut list = match &args[args.len() - 1] {
			Value::List(list) => list.clone(),
			other => List::new().push_front(other.clone())
		};
		for item in args[..args.len() - 1].into_iter().rev() {
			list = list.push_front(item.clone());
		}
		Ok(Value::List(list))
	}
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

pub fn eq(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	if args.len() == 0 {
		Ok(Value::Number(1.into()))
	} else {
		let val = &args[0];
		for other in &args[1..] {
			if other != val {
				return Ok(Value::Number(0.into()));
			}
		}
		Ok(Value::Number(1.into()))
	}
}

pub fn ne(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	if args.len() == 0 {
		Ok(Value::Number(0.into()))
	} else {
		let val = &args[0];
		for other in &args[1..] {
			if other == val {
				return Ok(Value::Number(0.into()));
			}
		}
		Ok(Value::Number(1.into()))
	}
}

pub fn lt(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	match args.len() {
		0 => Ok(Value::Number(0.into())),
		1 => Ok(Value::Number( (if args[0].to_number() < 0.into() { 1 } else { 0 }).into() )),
		_ => Ok(Value::Number( (if args[0].to_number() < args[1].to_number() { 1 } else { 0 }).into() ))
	}
}

pub fn le(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	match args.len() {
		0 => Ok(Value::Number(0.into())),
		1 => Ok(Value::Number( (if args[0].to_number() <= 0.into() { 1 } else { 0 }).into() )),
		_ => Ok(Value::Number( (if args[0].to_number() <= args[1].to_number() { 1 } else { 0 }).into() ))
	}
}

pub fn gt(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	match args.len() {
		0 => Ok(Value::Number(0.into())),
		1 => Ok(Value::Number( (if args[0].to_number() > 0.into() { 1 } else { 0 }).into() )),
		_ => Ok(Value::Number( (if args[0].to_number() > args[1].to_number() { 1 } else { 0 }).into() ))
	}
}

pub fn ge(_abl: &mut State, args: &[Value]) -> Result<Value, Error> {
	match args.len() {
		0 => Ok(Value::Number(0.into())),
		1 => Ok(Value::Number( (if args[0].to_number() >= 0.into() { 1 } else { 0 }).into() )),
		_ => Ok(Value::Number( (if args[0].to_number() >= args[1].to_number() { 1 } else { 0 }).into() ))
	}
}
