#![no_std]
#![allow(clippy::missing_safety_doc)]  // Our docs are in rST, not Rust Markup Documentation Format.

#![feature(sync_unsafe_cell)]
#![feature(linkage)]

pub mod raw;
pub mod interrupt;
pub mod critical;
pub mod runtime;

pub mod gx;
pub mod gx2d;

use core::arch::global_asm;

global_asm!(include_str!("start.s"));
