# Nominatim-rs

Nominatim-rs is a rust binding for the [Nominatim](https://nominatim.org/) api. It do reverse geolocation (finding a place from coordinates), lookup and search.

***Please conform to the Nominatim [ToS](https://operations.osmfoundation.org/policies/nominatim/)***

## Example

This exaple searches for a location and prints out the JSON

```rust
#[tokio::main]
async fn main() {
    let client = nominatim::Client::new(
        url::Url::parse("https://nominatim.openstreetmap.org/").unwrap(),
        "nominatim-rust/0.1.0 test-suite".to_string(),
        Some("john_t@mailo.com".to_string()),
    )
    .unwrap();

    let search = client
        .search(
            nominatim::SearchQueryBuilder::default()
                .address_details(true)
                .location_query(nominatim::LocationQuery::Generalised {
                    q: "bakery in berlin wedding".to_string(),
                })
                .limit(Some(1))
                .build()
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{}", serde_json::to_string_pretty(&search).unwrap());
}
```
