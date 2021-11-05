pub mod cli;
mod github;

use anyhow::Result;
use cli::Options;
use dotenv::dotenv;
use futures::future::join_all;
use github::{Contributor, GitHubResponse, Repo};
use reqwest::{
	Client,
	header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;
use std::{env, ops::Index};

#[derive(Deserialize, Debug)]
pub struct Bus {
	/// Total number of contributions the user has commited to the project
	pub contributions: usize,
	/// Name of the repository
	pub name: String,
	/// Number of stars the repository has
	pub stars: usize,
	/// The GitHub user / org who owns the repository
	pub user: String,
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

/// Search for repositories from a provided language & count & return as a
/// collection of `Bus` objects
pub async fn get_buses(options: &Options) -> Result<Vec<Bus>> {
	dotenv().ok();

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

	let language_filter = format!("language:{}", options.language);
	let repos = search_top_repos(&client, language_filter, options.count).await?;

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

	let default_contributor = Contributor {
		contributions: 0,
		login: String::from("Unknown User"),
	};

	let buses = repos
		.iter()
		.enumerate()
		.map(|(i, repo)| {
			// Assumes the first item in the collection is the top contributor
			// Docs: https://docs.github.com/en/rest/reference/repos#list-repository-contributors
			let top_contributor = contributors
				.index(i)
				.first()
				.unwrap_or(&default_contributor);

			Bus {
				contributions: top_contributor.contributions,
				name: repo.name.clone(),
				stars: repo.stargazers_count,
				user: repo.owner.login.clone(),
			}
		})
		.collect();

	Ok(buses)
}
