mod github;

use anyhow::Result;
use dotenv::dotenv;
use futures::future::join_all;
use github::{Contributor, GitHubResponse, Repo};
use reqwest::{
	header::{HeaderMap, HeaderValue},
	Client,
};
use std::{env, ops::Index};
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

/// Search for the most popular projects on GitHub by stars using a provided
/// search filter
async fn search_top_repos(
	client: &Client,
	filter: String,
	count: Option<usize>,
) -> Result<Vec<Repo>> {
	let per_page = count.unwrap_or(10);

	let request_url = format!(
		"https://api.github.com/search/repositories?q={}&per_page={}&sort=stars",
		filter, per_page
	);

	let repo_response = client.get(&request_url).send().await?;

	let status = repo_response.status();
	if status != 200 {
		panic!(
			"[{}] Failed to fetch repository data | {}",
			status,
			repo_response.text().await?
		);
	}

	let repos: GitHubResponse<Repo> = repo_response.json().await?;

	Ok(repos.items)
}

/// Fetch all contributors for a provided GitHub repository
async fn handle_contributor_response(client: &Client, repo: &Repo) -> Result<Vec<Contributor>> {
	let response = client.get(&repo.contributors_url).send().await?;

	let status = response.status();
	if status != 200 {
		let response_text = response.text().await?;
		panic!(
			"[{}] Failed to unwrap contributor response | {}",
			status, response_text
		);
	}

	let contributors: Vec<Contributor> = response.json().await?;

	Ok(contributors)
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	let opt = Options::from_args();

	let language_filter = format!("language:{}", opt.language);

	// Auth via PAT requires a prefix for its header value.
	// Docs: https://docs.github.com/en/rest/overview/other-authentication-methods#via-oauth-and-personal-access-tokens
	let personal_access_token = format!("token {}", env::var("GITHUB_ACCESS_TOKEN")?);

	let authorization_header_key = HeaderValue::from_str(&personal_access_token)?;
	let mut headers = HeaderMap::new();
	headers.insert("authorization", authorization_header_key);

	let client = Client::builder()
		.user_agent("Reqwest/bus-factor")
		.default_headers(headers)
		.build()?;

	let repos = search_top_repos(&client, language_filter, opt.count).await?;

	let contributors_results = join_all(
		repos
			.iter()
			.map(|repo| handle_contributor_response(&client, repo)),
	)
	.await;
	let contributors = contributors_results
		.into_iter()
		.map(|c| c.expect("Failed to unwrap contributor"))
		.collect::<Vec<Vec<Contributor>>>();

	println!("┌───────────────────────────────┬───────────────────────────┬─────────────────┬─────────────────┐");
	println!(
		"│{0: <30} │ {1: <25} │ {2: <15} │ {3: <15} │",
		"Project", "Top Contributor", "Percentage", "Stars"
	);
	println!("├───────────────────────────────┼───────────────────────────┼─────────────────┼─────────────────┤");

	repos.iter().enumerate().for_each(|(i, repo)| {
		let top_contributor = match contributors.index(i).first() {
			Some(val) => val,
			None => panic!(""),
		};

		println!(
			"│{0: <30} │ {1: <25} │ {2: <15} │ {3: <15} │",
			repo.name, top_contributor.login, top_contributor.contributions, repo.stargazers_count,
		);
	});

	println!("└───────────────────────────────┴───────────────────────────┴─────────────────┴─────────────────┘");

	Ok(())
}
