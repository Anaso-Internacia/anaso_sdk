use bitflags::bitflags;
use serde::{Deserialize, Serialize};

pub mod id;
pub mod request;
pub mod response;

bitflags! {
    /// A set of flags representing the permissions a user has in a given Anaseto
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
    pub struct Permissions: u32 {
        /// Can add/remove moderators
        const ADMIN = 0b00000001;

        /// Can delete any post/comment
        const DELETE_POSTS = 0b00000010;

        /// Can ban any user
        const BAN_USERS = 0b00000100;

        /// Can pin posts to the top of the Anaseto
        const PIN_POSTS = 0b00001000;

        /// Can post to the Anaseto
        const POST = 0b00010000;

        /// Can comment on posts in the Anaseto
        const COMMENT = 0b00100000;
    }
}

/// Used for determining allowed characters in things like usernames and anasetoj.
pub const ALPHABET: [char; 54] = [
    'a', 'b', 'c', 'ĉ', 'd', 'e', 'f', 'g', 'ĝ', 'h', 'ĥ', 'i', 'j', 'ĵ', 'k', 'l', 'm', 'n', 'o',
    'p', 'r', 's', 'ŝ', 't', 'u', 'v', 'z', 'A', 'B', 'C', 'Ĉ', 'D', 'E', 'F', 'G', 'Ĝ', 'H', 'Ĥ',
    'I', 'J', 'Ĵ', 'K', 'L', 'M', 'N', 'O', 'P', 'R', 'S', 'Ŝ', 'T', 'U', 'V', 'Z',
];
