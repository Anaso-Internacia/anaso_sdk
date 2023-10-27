macro_rules! gen_id {
    ($name:ident) => {
        #[derive(Copy, Clone, Debug, serde::Deserialize, serde::Serialize)]
        pub struct $name(pub i64);
    };
}

gen_id!(AnasejoId);
gen_id!(PostId);
gen_id!(UserId);
