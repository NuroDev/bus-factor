use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Bus Factor")]
struct Options {
	/// Max number of projects to fetch
	#[structopt(short, long)]
	count: u8,

	/// Name of the programming language
	#[structopt(short, long)]
	language: String,
}

#[tokio::main]
async fn main() -> Result<()> {
	let opt = Options::from_args();

	let language_filter = format!("language:{}", opt.language);

	let search_result = octocrab::instance()
		.search()
		.repositories(&language_filter)
		.sort("stars")
		.order("desc")
		.per_page(opt.count)
		.send()
		.await?;

	println!("┌───────────────────────────────┬───────────────────────────┬────────────┐");
	println!(
		"│{0: <30} │ {1: <25} │ {2: <10} │",
		"Project", "User", "Percentage"
	);
	println!("├───────────────────────────────┼───────────────────────────┼────────────┤");

	search_result.items.iter().for_each(|repo| {
		println!(
			"│{0: <30} │ {1: <25} │ {2: <10} │",
			repo.name, repo.owner.login, 0
		);
	});

	println!("└───────────────────────────────┴───────────────────────────┴────────────┘");

	Ok(())
}
