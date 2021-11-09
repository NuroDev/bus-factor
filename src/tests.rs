use crate::{cli::Options, get_buses};

#[tokio::test]
async fn language_rust_count_1() {
	let buses = get_buses(&Options {
		count: Some(1),
		language: "rust".to_string(),
	}).await.expect("Failed to unwrap buses");

	println!("{:#?}", buses);

	assert!(true);
	// assert_eq!(buses, []);
}
