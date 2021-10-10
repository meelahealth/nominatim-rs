pub mod search;
pub mod util;
pub mod types;
pub mod error;
pub mod serde_utils;
pub mod client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
