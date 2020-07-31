#![feature(box_syntax)]
#![allow(dead_code)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate rand;

pub mod renderer;
pub mod scenes;
pub mod tracer;
