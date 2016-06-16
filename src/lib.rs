#![cfg_attr(feature="lint", feature(plugin))]
#![cfg_attr(feature="lint", plugin(clippy))]
#![cfg_attr(not(feature="lint"), allow(unknown_lints))]
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
