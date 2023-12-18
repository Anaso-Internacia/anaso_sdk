macro_rules! gen_id {
    ($name:ident) => {
        #[derive(Copy, Clone, Debug, serde::Deserialize, serde::Serialize)]
        pub struct $name(pub i64);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                bs58::encode(self.0.to_le_bytes()).into_string().fmt(f)
            }
        }

        impl std::str::FromStr for $name {
            type Err = bs58::decode::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut out = [0; 8];
                bs58::decode(s).onto(&mut out)?;
                Ok(Self(i64::from_le_bytes(out)))
            }
        }
    };
}

pub use anaseto_id::AnasetoId;
pub use post_id::PostId;
pub use user_id::UserId;

pub mod anaseto_id;
pub mod post_id;
pub mod user_id;

#[cfg(test)]
mod test {
    use super::PostId;

    #[test]
    fn test_base58_conversion() {
        let post_id = PostId(12345);
        let s = post_id.to_string();
        let new_post_id: PostId = s.parse().unwrap();
        assert_eq!(post_id.0, new_post_id.0);
    }
}
