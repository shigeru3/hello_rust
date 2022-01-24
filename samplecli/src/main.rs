use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
	name = "My RPN program",
	version = "1.0.0",
	author = "shigeru",
	about = "Super awesome sample RPN calculator"
)]
struct Opts {
	/// Sets the level of verbosity
	#[clap(short, long)]
	verbose: bool,

	/// Number
	#[clap(name = "NUMBER")]
	num: i32,

	/// Formulas written in RPN
	#[clap(name = "FILE")]
	formula_file: Option<String>,
}

fn main() {
	let opts = Opts::parse();

	match opts.formula_file {
		Some(file) => println!("File specified: {}", file),
		None => println!("No file specified."),
	}
	println!("Is verbosity specified?: {}", opts.verbose);
}
