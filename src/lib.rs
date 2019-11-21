#![feature(box_syntax, box_patterns)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate rand;

pub mod renderer;
pub mod scenes;
pub mod tracer;
