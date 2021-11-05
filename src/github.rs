use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Owner {
	/// The GitHub user / org who owns the repository
	pub login: String,
}

#[derive(Deserialize, Debug)]
pub struct Repo {
	/// Name of the repository
	pub name: String,
	/// Repository owner metadat
	pub owner: Owner,
	/// Number of stars the repository has
	pub stargazers_count: usize,
}

#[derive(Deserialize, Debug)]
pub struct Contributor {
	/// The number of contributions to the project
	pub contributions: usize,
	/// The users GitHub username
	pub login: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubResponse <T> {
	pub items: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct GitHubError {
	pub message: String,
}
