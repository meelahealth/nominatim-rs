use std::str::FromStr;
use std::fmt;
use std::convert::Infallible;

/// Stores a point, longitude and latitude.
pub type Point = (f64, f64);

/// Stores two points, longitude and latitude. Any two corner points of the
/// box are accepted as long as they span a real box. `x` is longitude,
/// `y` is latitude.
pub type ViewBox = (Point, Point);

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
