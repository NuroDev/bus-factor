use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Bus Factor")]
pub struct Options {
	/// Max number of projects to fetch
	#[structopt(short, long)]
	pub count: Option<usize>,

	/// Name of the programming language
	#[structopt(short, long)]
	pub language: String,
}
