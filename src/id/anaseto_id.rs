#[cfg(feature = "client")]
use crate::{models::response::AnasetoData, Error, Result};

gen_id!(AnasetoId);

#[cfg(feature = "client")]
impl AnasetoId {
    pub async fn to_anaseto(&self) -> Result<AnasetoData> {
        Err(Error::E)
    }
}
