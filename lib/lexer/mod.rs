use nom::{
    branch::alt,
    // bytes::complete::{tag, tag_no_case, take, take_while},
    bytes::complete::tag,
    // bytes::complete::{tag, tag_no_case, take_while},
    combinator::value,
    // character::complete::{alpha1, alphanumeric1, one_of},
    // combinator::opt,
    // error::{context, ErrorKind, VerboseError},
    IResult,
    // number::complete::i16,
    sequence::tuple
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

#[cfg(test)]
mod tests {
    
}
