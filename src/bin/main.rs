use anyhow::Result;
use bus_factor::{cli::Options, get_buses};
use dotenv::dotenv;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	let opt = Options::from_args();

	let buses = get_buses(&opt).await?;

	println!("┌───────────────────────────────┬───────────────────────────┬─────────────────┬─────────────────┐");
	println!(
		"│{0: <30} │ {1: <25} │ {2: <15} │ {3: <15} │",
		"Project", "Top Contributor", "Percentage", "Stars"
	);
	println!("├───────────────────────────────┼───────────────────────────┼─────────────────┼─────────────────┤");

	buses.iter().for_each(|bus| {
		println!(
			"│{0: <30} │ {1: <25} │ {2: <15} │ {3: <15} │",
			bus.name, bus.user, bus.contributions, bus.stars,
		);
	});

	println!("└───────────────────────────────┴───────────────────────────┴─────────────────┴─────────────────┘");

	Ok(())
}
