#![allow(dead_code)]

pub mod server;
pub mod systeminfo;
pub mod virtual_memory;
// pub mod wm;
// added for experimental use
pub mod kinfo;
pub mod mail;
pub const BANNER: &str = include_str!("banner.txt");
