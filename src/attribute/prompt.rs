use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::char,
    character::complete::{alphanumeric1, multispace1, one_of},
    combinator::{map, opt, recognize},
    multi::many1,
    sequence::{delimited, tuple},
    IResult,
};
use serde::Serialize;

use crate::{util::ws, KconfigInput};

use super::expression::{parse_if_expression_attribute, Expression};

pub fn parse_prompt(input: KconfigInput) -> IResult<KconfigInput, Prompt> {
    map(
        tuple((
            ws(tag("prompt")),
            parse_prompt_option,
            opt(parse_if_expression_attribute),
        )),
        |(_, p, i)| Prompt {
            prompt: p.to_string(),
            r#if: i,
        },
    )(input)
}

pub fn parse_prompt_option(input: KconfigInput) -> IResult<KconfigInput, &str> {
    map(
        alt((
            delimited(
                ws(char('"')),
                recognize(ws(many1(alt((
                    alphanumeric1,
                    multispace1,
                    tag("\\\""),
                    recognize(one_of("&#*|!É{}<>[]()+'=,:;μ-?._$/")),
                ))))),
                char('"'),
            ),
            delimited(ws(char('\'')), take_until("'"), char('\'')),
        )),
        |d: KconfigInput| d.fragment().to_owned(),
    )(input)
}

#[derive(Debug, Clone, Serialize, PartialEq)]

pub struct Prompt {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Expression>,
}