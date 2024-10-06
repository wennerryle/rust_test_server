mod http_code;
mod http_methods;

use http_methods::HTTPMethods;
use nom::{
    bytes::complete::{tag, take_till},
    character::is_space,
    combinator::map_res,
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

fn till_space(s: &str) -> IResult<&str, &str> {
    take_till(|c| c == ' ')(s)
}

pub fn p_method(i: &str) -> IResult<&str, Option<HTTPMethods>> {
    let (rest, res) = till_space(i)?;
    let res = HTTPMethods::from(res);

    Ok((rest, res))
}
