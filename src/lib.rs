pub mod cli;
mod github;
#[cfg(test)]
mod tests;

use anyhow::Result;
use cli::Options;
use dotenv::dotenv;
use futures::future::join_all;
use github::{Contributor, GitHubResponse, Repo};
use reqwest::{
	header::{HeaderMap, HeaderValue},
	Client,
};
use std::{env, ops::Index};

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
pub async fn get_buses(options: &Options) -> Result<Vec<(String, Contributor, usize, usize)>> {
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

	let contributors_total_commits = contributors
		.iter()
		.map(|repo| {
			let mut total_commits = 0;
			repo.iter()
				.for_each(|contributor| total_commits += contributor.contributions);

			total_commits
		})
		.collect::<Vec<usize>>();

	let buses = repos
		.iter()
		.enumerate()
		.filter_map(|(i, repo)| {
			// Filters out any contributors that are not valid / viable &
			// returns the top contributor to the project & their contribution
			// percentage.
			// Assumes the first item in the collection is the top contributor
			// (https://docs.github.com/en/rest/reference/repos#list-repository-contributors)
			let (top_contributor, percentage) = contributors
				.index(i)
				.into_iter()
				.filter_map(|contributor| {
					let percentage =
						(100 * contributor.contributions) / contributors_total_commits.index(i);

					// Ignore any active developers who's contributions are over 75%
					if percentage >= 75 {
						return None;
					}

					Some((contributor.clone(), percentage))
				})
				.collect::<Vec<(Contributor, usize)>>()
				.first()
				.unwrap_or(&(
					Contributor {
						contributions: 0,
						login: String::from("Unknown User"),
					},
					0,
				))
				.clone();

			Some((
				format!("{}/{}", repo.owner.login, repo.name),
				top_contributor,
				percentage,
				repo.stargazers_count,
			))
		})
		.collect();

	Ok(buses)
}
