use anyhow::Result;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;
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

#[derive(Deserialize, Debug)]
struct Owner {
	login: String,
}

#[derive(Deserialize, Debug)]
struct Repo {
	name: String,
	owner: Owner,
	stargazers_count: u32,
}

#[derive(Deserialize, Debug)]
struct GitHubResponse {
	items: Vec<Repo>,
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	let opt = Options::from_args();

	let language_filter = format!("language:{}", opt.language);

	let request_url = format!(
		"https://api.github.com/search/repositories?q={}&per_page={}&sort=stars",
		language_filter, opt.count
	);

	// Auth via PAT requires a prefix for its header value.
	// See: https://docs.github.com/en/rest/overview/other-authentication-methods#via-oauth-and-personal-access-tokens
	let personal_access_token_var = env::var("GITHUB_ACCESS_TOKEN")?;
	let personal_access_token = format!("token {}", &personal_access_token_var);

	let response: GitHubResponse = Client::new()
		.get(&request_url)
		.header("User-Agent", "Reqwest/bus-factor")
		.header("authorization", &personal_access_token)
		.send()
		.await?
		.json()
		.await?;

	println!("┌───────────────────────────────┬───────────────────────────┬────────────┐");
	println!(
		"│{0: <30} │ {1: <25} │ {2: <10} │",
		"Project", "User", "Percentage"
	);
	println!("├───────────────────────────────┼───────────────────────────┼────────────┤");

	response.items.iter().for_each(|repo| {
		println!(
			"│{0: <30} │ {1: <25} │ {2: <10} │",
			repo.name, repo.owner.login, repo.stargazers_count
		);
	});

	println!("└───────────────────────────────┴───────────────────────────┴────────────┘");

	Ok(())
}
