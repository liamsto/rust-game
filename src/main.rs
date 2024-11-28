#![feature(trait_upcasting)]

use core::panic;
use std::process::exit;

use assets::{all_enemies, all_items, all_moves};
use battle::Battle;
use combatant::Combatant;
use std::net::TcpListener;
extern crate lazy_static;

mod assets;
mod battle;
mod character;
mod cloneablefn;
mod cloneablefn_combatant;
mod combatant;
mod effect;
mod enemy;
mod item;
mod move_mod;
mod server;

fn main() {

}
