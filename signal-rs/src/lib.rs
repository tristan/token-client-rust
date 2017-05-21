#![feature(plugin)]
#![plugin(mod_path)]

extern crate rustc_serialize;
extern crate rand;
extern crate crypto;
extern crate protobuf;
extern crate time;

pub mod curve;
pub mod keys;
pub mod session;
pub mod message;
#[macro_use]
pub mod protocol;
pub mod ratchet;
pub mod state;
pub mod cipher;

mod_path! LocalStorageProtocol (concat!(env!("OUT_DIR"), "/LocalStorageProtocol.rs"));
mod_path! WhisperTextProtocol (concat!(env!("OUT_DIR"), "/WhisperTextProtocol.rs"));
pub_mod_path! SignalService (concat!(env!("OUT_DIR"), "/SignalService.rs"));

#[cfg(feature = "sqlite-store")]
pub mod sqlite_store;

#[cfg(test)]
pub mod tests;
