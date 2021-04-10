use nom::{
    branch::alt,
    // bytes::complete::{tag, tag_no_case, take, take_while},
    bytes::complete::tag,
    // bytes::complete::{tag, tag_no_case, take_while},
    combinator::value,
    multi::many0,
    // character::complete::{alpha1, alphanumeric1, one_of},
    // combinator::opt,
    // error::{context, ErrorKind, VerboseError},
    IResult,
    // number::complete::i16,
    sequence::tuple
};

pub fn sumsub_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("+"), tag("-")))(input)
}

pub fn multdiv_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("/"), tag("*")))(input)
}

pub fn op_relacional_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("<="), tag("=="), tag(">="), tag("!="), tag("<"), tag(">")))(input)
}

pub fn op_logica_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("&"), tag("|")))(input)
}

pub fn leer_parser(input: &str) -> IResult<&str, (&str, &str, &str, Vec<(&str, &str)>, &str)> {
    tuple((tag("lee"), tag("("),tag("id"), 
        many0(tuple((
            tag(","),
            tag("id")
        ))),tag(")")
    ))
    (input)
}

pub fn tipo_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("entero"), tag("flotante"), tag("char")))(input)
}

// pub fn ids(input: &str) -> Res<&str, Vec<& str>> {
//     context(
//         "ids",
//         tuple((
//             url_code_points,
//             many0(tuple((
//                 space,
//                 tag(","),
//                 space,
//                 url_code_points,
//             ))),
//         )),
//     )(input)
//     .map(|(next_input, res)| {
//         let mut qps = Vec::new();
//         qps.push(res.0);
//         for qp in res.1 {
//             qps.push(qp.3);
//         }
//         (next_input, qps)
//     })
// }

pub fn arit(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((sumsub_parser, multdiv_parser))(input)
}

#[cfg(test)]
mod tests {
    
}
