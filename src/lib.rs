#![feature(associated_consts)]

#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]
#![cfg_attr(not(test), allow(unknown_lints))]
#[macro_use]
// TODO: Track bitflags#20 for namespaced flags
extern crate bitflags;

pub mod instance;
pub mod device;
pub mod sys;
pub mod debug;
pub mod command_pool;
pub mod command_buffer;
pub mod fence;
pub mod semaphore;
pub mod event;
