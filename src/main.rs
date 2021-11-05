mod github;

use anyhow::Result;
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use structopt::StructOpt;

use crate::github::{GitHubResponse, GitHubResponseError, Repo};

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

	let request_response = Client::new()
		.get(&request_url)
		.header("User-Agent", "Reqwest/bus-factor")
		.header("authorization", &personal_access_token)
		.send()
		.await?;

	if request_response.status() != 200 {
		let error_response: GitHubResponseError = request_response.json().await?;
		panic!(
			"Failed to fetch repository data | {}",
			error_response.errors[0].message
		);
	}

	let response: GitHubResponse<Repo> = request_response.json().await?;

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
