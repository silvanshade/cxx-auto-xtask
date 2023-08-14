mod build;
mod check;
mod clang;
mod clippy;
mod cmake;
mod doc;
mod edit;
mod fmt;
mod miri;
mod tarpaulin;
mod test;
mod udeps;
mod valgrind;

pub use self::{
    build::build,
    check::check,
    clang::clang,
    clippy::clippy,
    cmake::cmake,
    doc::doc,
    edit::edit,
    fmt::fmt,
    miri::miri,
    tarpaulin::tarpaulin,
    test::test,
    udeps::udeps,
    valgrind::valgrind,
};

use crate::config::Config;
use std::{ffi::OsString, path::PathBuf};

pub struct Context<'a> {
    pub config: &'a Config,
    pub args: &'a mut pico_args::Arguments,
    pub tool_args: Vec<OsString>,
    pub current_dir: Option<PathBuf>,
    pub subcommand: Option<String>,
}

impl<'a> Context<'a> {
    pub fn new(config: &'a Config, args: &'a mut pico_args::Arguments, tool_args: Vec<OsString>) -> Context<'a> {
        Context {
            config,
            args,
            tool_args,
            current_dir: None,
            subcommand: None,
        }
    }
}
