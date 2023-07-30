//! # steam-rs
//!
//! The `steam-rs` crate provides convenient Rust bindings for the Steam Web API.

pub mod app_id;
pub mod player_service;
pub mod steam_id;
pub mod steam_news;
pub mod steam_user;
pub mod steam_user_stats;

mod errors; // This remains private - maybe
mod macros; // This remains private

pub const BASE: &str = "https://api.steampowered.com";

pub struct Steam {
    api_key: String,
}

impl Steam {
    pub fn new(api_key: &str) -> Steam {
        Steam { api_key: api_key.to_string() }
    }
}