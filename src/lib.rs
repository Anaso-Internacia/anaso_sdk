#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod id;

#[cfg(feature = "models")]
pub mod models;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub enum Error {
    E,
}
