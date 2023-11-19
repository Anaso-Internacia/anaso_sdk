pub mod id;
pub mod request;
pub mod response;

/// Used for determining allowed characters in things like usernames and anasetoj.
pub const ALPHABET: [char; 54] = [
    'a', 'b', 'c', 'ĉ', 'd', 'e', 'f', 'g', 'ĝ', 'h', 'ĥ', 'i', 'j', 'ĵ', 'k', 'l', 'm', 'n', 'o',
    'p', 'r', 's', 'ŝ', 't', 'u', 'v', 'z', 'A', 'B', 'C', 'Ĉ', 'D', 'E', 'F', 'G', 'Ĝ', 'H', 'Ĥ',
    'I', 'J', 'Ĵ', 'K', 'L', 'M', 'N', 'O', 'P', 'R', 'S', 'Ŝ', 'T', 'U', 'V', 'Z',
];
