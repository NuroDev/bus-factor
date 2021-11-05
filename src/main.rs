mod github;

use anyhow::Result;
use dotenv::dotenv;
use github::{GitHubResponse, Repo};
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use std::env;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Bus Factor")]
struct Options {
	/// Max number of projects to fetch
	#[structopt(short, long)]
	count: Option<usize>,

	/// Name of the programming language
	#[structopt(short, long)]
	language: String,
}

/// Search for the most popular projects on GitHub by stars using a provided search filter
async fn search_top_repos (client: &Client, filter: String, count: Option<usize>) -> Result<Vec<Repo>> {
	let per_page = match count {
		Some(c) => c,
		None => 10,
	};

	let request_url = format!(
		"https://api.github.com/search/repositories?q={}&per_page={}&sort=stars",
		filter, per_page
	);

	let repo_response = client
		.get(&request_url)
		.send()
		.await?;

	let status = repo_response.status();
	if status != 200 {
		panic!("[{}] Failed to fetch repository data | {}", status, repo_response.text().await?);
	}

	let repos: GitHubResponse<Repo> = repo_response.json().await?;

	Ok(repos.items)
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	let opt = Options::from_args();

	let language_filter = format!("language:{}", opt.language);

	// Auth via PAT requires a prefix for its header value.
	// Docs: https://docs.github.com/en/rest/overview/other-authentication-methods#via-oauth-and-personal-access-tokens
	let personal_access_token_var = env::var("GITHUB_ACCESS_TOKEN")?;
	let personal_access_token = format!("token {}", &personal_access_token_var);

	let authorization_header_key = HeaderValue::from_str(&personal_access_token)?;
	let mut headers = HeaderMap::new();
	headers.insert("authorization", authorization_header_key);

	let client = Client::builder().user_agent("Reqwest/bus-factor")
						.default_headers(headers)
						.build()?;

	let repos = search_top_repos(&client, language_filter, opt.count).await?;

	println!("┌───────────────────────────────┬───────────────────────────┬────────────┐");
	println!(
		"│{0: <30} │ {1: <25} │ {2: <10} │",
		"Project", "User", "Percentage"
	);
	println!("├───────────────────────────────┼───────────────────────────┼────────────┤");

	repos.iter().for_each(|repo| {
		println!(
			"│{0: <30} │ {1: <25} │ {2: <10} │",
			repo.name, repo.owner.login, repo.stargazers_count
		);
	});

	println!("└───────────────────────────────┴───────────────────────────┴────────────┘");

	Ok(())
}
