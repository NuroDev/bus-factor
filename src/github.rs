use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Owner {
	pub login: String,
}

#[derive(Deserialize, Debug)]
pub struct Repo {
	pub name: String,
	pub owner: Owner,
	pub stargazers_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct GitHubResponse {
	pub items: Vec<Repo>,
}

#[derive(Deserialize, Debug)]
pub struct GitHubError {
	pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubResponseError {
	pub errors: Vec<GitHubError>,
}
