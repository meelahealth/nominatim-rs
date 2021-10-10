use std::str::FromStr;
pub use serde::{Deserialize, Serialize};
pub use std::collections::HashMap;
use std::fmt;
use std::convert::Infallible;

pub enum OsmType {
    Node,
    Way,
    Relation,
    Other(String),
}


impl FromStr for OsmType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<OsmType, Self::Err> {
        Ok(
            match s {
                "node" => Self::Node,
                "way" => Self::Way,
                "relation" => Self::Relation,
                _ => Self::Other(s.to_string()),
            }
        )
    }
}

impl fmt::Display for OsmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Node => write!(f, "node"),
            Self::Way => write!(f, "way"),
            Self::Relation => write!(f, "relation"),
            Self::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Country {
    Country(String),
    CountryCode(String),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    Region(String),
    State(String),
    StateDistrict(String),
    County(String),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Municipality {
    Region(String),
    State(String),
    StateDistrict(String),
    County(String),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CityDistrict {
    CityDistrict(String),
    District(String),
    Borough(String),
    Suburb(String),
    Subdivision(String),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Hamlet {
    Hamlet(String),
    Croft(String),
    IsolatedDwelling(String),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Neighbourhood {
    Neighbourhood(String),
    Allotments(String),
    Quarter(String),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CityBlock {
    CityBlock(String),
    Residental(String),
    Farm(String),
    Farmyard(String),
    Industrial(String),
    Commercial(String),
    Retail(String),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum House {
    HouseNumber(u64),
    HouseName(String),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Place {
    Emergency(String),
    Historic(String),
    Military(String),
    Natural(String),
    Landuse(String),
    Place(String),
    Railway(String),
    ManMade(String),
    Aerialway(String),
    Boundary(String),
    Amenity(String),
    Aeroway(String),
    Club(String),
    Leisure(String),
    Office(String),
    MoutainPass(String),
    Shop(String),
    Tourism(String),
    Bridge(String),
    Tunnel(String),
    Waterway(String),
}

#[derive(Deserialize, Serialize)]
pub struct Address {
    pub continent: Option<String>,
    #[serde(flatten)]
    pub country: Option<Country>,
    #[serde(flatten)]
    pub region: Option<Region>,
    #[serde(flatten)]
    pub municipality: Option<Municipality>,
    #[serde(flatten)]
    pub city_district: Option<CityDistrict>,
    #[serde(flatten)]
    pub hamlet: Option<Hamlet>,
    #[serde(flatten)]
    pub neighbourhood: Option<Neighbourhood>,
    #[serde(flatten)]
    pub city_block: Option<CityBlock>,
    pub road: Option<String>,
    #[serde(flatten)]
    pub house: Option<House>,
    #[serde(flatten)]
    pub place: Option<Place>,
    pub postcode: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    /// Reference to the Nominatim internal database ID.
    #[serde(deserialize_with= "crate::serde_utils::deserialize_from_string_opt")]
    #[serde(serialize_with = "crate::serde_utils::serialize_as_string_opt")]
    pub place_id: Option<u64>,
    /// The type of this response. Likely a `node`, `way` or `relation`.
    pub osm_type: Option<String>,
    /// Reference to the OSM object
    pub osm_id: Option<String>,
    #[serde(deserialize_with= "crate::serde_utils::deserialize_from_string_opt")]
    #[serde(serialize_with = "crate::serde_utils::serialize_as_string_opt")]
    /// Longitude of the centroid of the object
    pub lon: Option<f64>,
    #[serde(deserialize_with= "crate::serde_utils::deserialize_from_string_opt")]
    #[serde(serialize_with = "crate::serde_utils::serialize_as_string_opt")]
    /// Latitude of the centroid of the object
    pub lat: Option<f64>,
    /// A license
    pub license: Option<String>,
    /// Dictionary of address details.
    pub address: Option<Address>,
    /// Full comma-separated address
    pub display_name: Option<String>,
    /// Link to class icon (if available)
    pub icon: Option<String>,
    /// The main OSM tag
    pub class: Option<String>,
    /// The main OSM tag
    pub r#type: Option<String>,
    #[serde(deserialize_with= "crate::serde_utils::deserialize_from_string_opt")]
    #[serde(serialize_with = "crate::serde_utils::serialize_as_string_opt")]
    /// Computed importance rank
    pub importance: Option<f64>,
}
