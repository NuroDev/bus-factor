use anyhow::Result;
use bus_factor::{cli::Options, get_buses};
use stopwatch::Stopwatch;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
	let opt = Options::from_args();

	let mut sw = Stopwatch::start_new();
	let buses = get_buses(&opt).await?;
	sw.stop();

	println!("┌───────────────────────────────┬───────────────────────────┬─────────────────┬─────────────────┐");
	println!(
		"│{0: <30} │ {1: <25} │ {2: <15} │ {3: <15} │",
		"Project", "Top Contributor", "Percentage", "Stars"
	);
	println!("├───────────────────────────────┼───────────────────────────┼─────────────────┼─────────────────┤");

	buses.iter().for_each(|(name, stars, top_contributor)| {
		println!(
			"│{0: <30} │ {1: <25} │ {2: <15} │ {3: <15} │",
			name, top_contributor.login, top_contributor.contributions, stars,
		);
	});

	let elapsed_ms = format!("{}ms", sw.elapsed_ms());

	println!("├───────────────────────────────┴───────────────────────────┴─────────────────┴─────────────────┤");
	println!("│ Completed in: {0: <80}│", elapsed_ms);
	println!("└───────────────────────────────────────────────────────────────────────────────────────────────┘");

	Ok(())
}
