mod github;

use anyhow::Result;
use dotenv::dotenv;
use reqwest::{
	Client,
	header::{HeaderMap, HeaderValue}
};
use std::env;
use structopt::StructOpt;

use crate::github::{GitHubResponse, Repo};

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

	let authorization_header_key = HeaderValue::from_str(&personal_access_token)?;

	let mut headers = HeaderMap::new();
	headers.insert("authorization", authorization_header_key);

	let client = Client::builder().user_agent("Reqwest/bus-factor").default_headers(headers).build()?;

	let repo_response = client
		.get(&request_url)
		.send()
		.await?;

	let status = repo_response.status();
	if status != 200 {
		panic!("[{}] Failed to fetch repository data | {}", status, repo_response.text().await?);
	}

	let repos: GitHubResponse<Repo> = repo_response.json().await?;

	println!("┌───────────────────────────────┬───────────────────────────┬────────────┐");
	println!(
		"│{0: <30} │ {1: <25} │ {2: <10} │",
		"Project", "User", "Percentage"
	);
	println!("├───────────────────────────────┼───────────────────────────┼────────────┤");

	repos.items.iter().for_each(|repo| {
		println!(
			"│{0: <30} │ {1: <25} │ {2: <10} │",
			repo.name, repo.owner.login, repo.stargazers_count
		);
	});

	println!("└───────────────────────────────┴───────────────────────────┴────────────┘");

	Ok(())
}
