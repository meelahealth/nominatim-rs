#[tokio::test]
async fn reverse() {
    let client = nominatim::Client::new(
        url::Url::parse("https://nominatim.openstreetmap.org/").unwrap(),
        "nominatim-rust/0.1.0 test-suite".to_string(),
        Some("john_t@mailo.com".to_string()),
    )
    .unwrap();

    let reverse = client
        .reverse(
            nominatim::ReverseQueryBuilder::default()
                .address_details(true)
                .lon(13.53918)
                .lat(52.5460941)
                .zoom(nominatim::Zoom::Building)
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{}", serde_json::to_string_pretty(&reverse).unwrap());

    let reverse_expected: nominatim::Response =
        serde_json::from_str(include_str!("./reverse_expected.json")).unwrap();

    assert_eq!(reverse, reverse_expected);
}
