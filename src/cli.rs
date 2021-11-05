use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Bus Factor")]
pub struct Options {
	/// Number of projects to search for
	#[structopt(short, long)]
	pub count: Option<usize>,

	/// Name of the programming language
	#[structopt(short, long)]
	pub language: String,
}
