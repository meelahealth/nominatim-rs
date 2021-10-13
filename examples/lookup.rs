#[tokio::main]
async fn main() {
    let client = nominatim::Client::new(
        url::Url::parse("https://nominatim.openstreetmap.org/").unwrap(),
        concat!(
            env!("CARGO_CRATE_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
            " ",
            "test-suite"
        )
        .to_string(),
        Some("john_t@mailo.com".to_string()),
    )
    .unwrap();

    let lookup = client
        .lookup(
            nominatim::LookupQueryBuilder::default()
                .osm_ids(vec!["N317179427".to_string()])
                .address_details(true)
                .extra_tags(true)
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{}", serde_json::to_string_pretty(&lookup).unwrap());
}
