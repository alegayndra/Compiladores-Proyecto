use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    IResult,
    sequence::tuple,
};

pub fn sumsub_parser(i: &str) -> IResult<&str, &str> {
    alt((tag("+"), tag("-")))(i)
}

pub fn multdiv_parser(i: &str) -> IResult<&str, &str> {
    alt((tag("/"), tag("*")))(i)
}


pub fn arit(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((sumsub_parser, multdiv_parser))(input)
}

pub fn id(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric())(input)
}

pub fn ws(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c == ' ')(input)
}

#[cfg(test)]
mod tests {
    
}
