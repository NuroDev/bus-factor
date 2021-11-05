use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Owner {
	/// The GitHub user / org who owns the repository
	pub login: String,
}

#[derive(Deserialize, Debug)]
pub struct Repo {
	/// API URL to fetch all contributors to the project
	pub contributors_url: String,
	/// Name of the repository
	pub name: String,
	/// Repository owner metadata
	pub owner: Owner,
	/// Number of stars the repository has
	pub stargazers_count: usize,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Contributor {
	/// The number of contributions to the project
	pub contributions: usize,
	/// The users GitHub username
	pub login: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubResponse<T> {
	pub items: Vec<T>,
}
