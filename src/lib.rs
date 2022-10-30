//! Rustagram - Apply instagram filters to your photos.
//!
//! ## Example
//!
//! ```rust,no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use rustagram::{RustagramFilter, FilterType};
//!
//! let img = image::open("myimage.png")?;
//! let modified = img.to_rgba8().apply_filter(FilterType::Valencia);
//! # Ok(())
//! # }
//! ```

use std::fmt;
use std::{fmt::Display, str::FromStr};

/// Re-export of the `image` crate.
pub use image;

pub use filters::{FilterType, RustagramFilter};

mod filters;
mod rustaops;

/// Failed to parse a valid filter from the given name.
#[derive(Debug)]
pub struct InvalidFilterName;

impl Display for InvalidFilterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl FromStr for FilterType {
    type Err = InvalidFilterName;

    fn from_str(filter: &str) -> Result<Self, Self::Err> {
        let search_result = AVAILABLE_FILTERS
            .iter()
            .find(|f| f.0.eq_ignore_ascii_case(filter));
        match search_result {
            Some((_, filter)) => Ok(*filter),
            None => Err(InvalidFilterName),
        }
    }
}

const AVAILABLE_FILTERS: &[(&str, FilterType)] = &[
    ("1977", FilterType::NineTeenSeventySeven),
    ("nineteenseventyseven", FilterType::NineTeenSeventySeven),
    ("aden", FilterType::Aden),
    ("brannan", FilterType::Brannan),
    ("brooklyn", FilterType::Brooklyn),
    ("clarendon", FilterType::Clarendon),
    ("earlybird", FilterType::Earlybird),
    ("gingham", FilterType::Gingham),
    ("hudson", FilterType::Hudson),
    ("inkwell", FilterType::Inkwell),
    ("kelvin", FilterType::Kelvin),
    ("lark", FilterType::Lark),
    ("lofi", FilterType::Lofi),
    ("maven", FilterType::Maven),
    ("mayfair", FilterType::Mayfair),
    ("moon", FilterType::Moon),
    ("nashville", FilterType::Nashville),
    ("reyes", FilterType::Reyes),
    ("rise", FilterType::Rise),
    ("slumber", FilterType::Slumber),
    ("stinson", FilterType::Stinson),
    ("toaster", FilterType::Toaster),
    ("valencia", FilterType::Valencia),
    ("walden", FilterType::Walden),
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses() {
        assert_eq!(FilterType::NineTeenSeventySeven, "1977".parse().unwrap());
        assert_eq!(FilterType::NineTeenSeventySeven, "NineTeenSeventySeven".parse().unwrap());
        assert_eq!(FilterType::NineTeenSeventySeven, "nineteenseventyseven".parse().unwrap());
    }
}
