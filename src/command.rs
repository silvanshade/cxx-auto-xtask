mod build;
mod check;
mod clang;
mod clippy;
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
    doc::doc,
    edit::edit,
    fmt::fmt,
    miri::miri,
    tarpaulin::tarpaulin,
    test::test,
    udeps::udeps,
    valgrind::valgrind,
};
