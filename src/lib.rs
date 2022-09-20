extern crate image;

pub mod filters;
mod rustaops;

use filters::FilterType;

pub fn validate_filter_type(
    filter: &str,
    filter_strings: &[&str],
    filter_types: &[FilterType],
) -> Result<FilterType, &'static str> {
    let search_result = filter_strings.iter().enumerate().find(|f| &filter == f.1);
    match search_result {
        Some((i, _)) => Ok(filter_types[i].clone()),
        None => Err("Invalid filter type"),
    }
}
