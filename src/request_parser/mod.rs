mod http_code;
mod http_methods;

use http_methods::HTTPMethods;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    error::ErrorKind,
    sequence::tuple,
    IResult,
};

type Header<'a> = (&'a str, &'a str);

#[derive(Debug, PartialEq)]
pub struct HTTPRequest<'a> {
    // life time will taken from request string
    pub method: HTTPMethods,
    pub route: &'a str,
    pub requested_headers: [Header<'a>],
}

pub fn p_method(i: &str) -> Result<(&str, HTTPMethods), nom::Err<()>> {
    let parsers = (
        tag::<&str, &str, ()>("GET"),
        tag("HEAD"),
        tag("POST"),
        tag("PUT"),
        tag("DELETE"),
        tag("CONNECT"),
        tag("TRACE"),
        tag("PATCH"),
    );

    map_res(alt(parsers), HTTPMethods::from)(i)
}
