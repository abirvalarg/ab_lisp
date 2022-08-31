mod action;
mod error;
mod location;
mod parser;
mod prelude;
mod state;
mod value;

fn run() -> Result<(), error::Error> {
	let args = std::env::args().collect::<Vec<_>>();
	if args.len() == 2 {
		let src = std::fs::read_to_string(&args[1])
			.map_err(|_| error::Error::new(error::ErrorKind::File(args[1].clone())))?;
		let mut abl = state::State::new();
		let source = location::Source::File(args[1].clone());
		let actions = parser::parse(source, &src)?;
		abl.reg_prelude();
		abl.execute(&actions[..])?;
		Ok(())
	} else {
		Err(error::Error::new(error::ErrorKind::Usage))
	}
}

fn main() {
	if let Err(err) = run() {
		eprintln!("{err}");
		std::process::exit(1);
	}
}
