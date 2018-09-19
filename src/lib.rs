#![warn(missing_docs)]

//! This lib is used to parse formatted strings to different types

use std::error::Error;
use std::fmt;
extern crate num_traits;

pub mod bytes;
pub mod num;

/// Error parsing formatted strings
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParseError {
    /// Parsing an empty string
    EmptyInput,

    /// The numeric value is missing
    MissingValue,

    /// Malformed numeric value
    InvalidValue,

    /// Unknown unit
    InvalidUnit,

    /// The numeric value is too large
    Overflow,
}

impl ParseError {
    fn description(&self) -> &str {
        match self {
            ParseError::EmptyInput => "empty input",
            ParseError::MissingValue => "missing value",
            ParseError::InvalidValue => "invalid value",
            ParseError::InvalidUnit => "invalid unit",
            ParseError::Overflow => "value overflow",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(self.description())
    }
}

impl Error for ParseError {}
