use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Owner {
	pub login: String,
}

#[derive(Deserialize, Debug)]
pub struct Repo {
	pub name: String,
	pub owner: Owner,
	pub stargazers_count: usize,
}

#[derive(Deserialize, Debug)]
pub struct GitHubResponse <T> {
	pub items: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct GitHubError {
	pub message: String,
}
