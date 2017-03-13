use super::super::rustc_serialize::base64::{Config,CharacterSet,Newline};

pub static STANDARD_NO_PADDING: Config = Config{
    char_set: CharacterSet::Standard,
    newline: Newline::CRLF,
    pad: false,
    line_length: None
};
