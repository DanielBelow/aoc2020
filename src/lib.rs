#![warn(nonstandard_style, rust_2018_idioms)]
#![allow(clippy::implicit_hasher)]

mod iterator_ext;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

aoc_runner_derive::aoc_lib! { year = 2020 }
