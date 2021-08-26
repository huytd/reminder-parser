use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1, multispace0};
use nom::combinator::{opt, rest, value};
use nom::IResult;
use nom::Parser;
use nom::{
    multi::many_till,
    sequence::{pair, tuple},
};
use std::str;

type ReminderTime<'a> = (&'a str, &'a str, Option<&'a str>);
type ReminderDate<'a> = (&'a str, bool);

#[derive(Debug)]
struct Reminder<'a> {
    text: String,
    date: ReminderDate<'a>,
    time: ReminderTime<'a>,
}

fn parse_time(input: &str) -> IResult<&str, ReminderTime> {
    let (input, (_, _, hour, opt_min, _, am)) = tuple((
        opt(tag("at")),
        multispace0,
        digit1,
        opt(tuple((tag(":"), digit1))),
        multispace0,
        opt(alt((tag("am"), tag("pm")))),
    ))
    .parse(input)?;

    let (_, min) = opt_min.unwrap_or(("", "00"));

    Ok((input, (hour, min, am)))
}

#[test]
fn test_parse_time() {
    let test_times = [
        ("at 11:00", Ok(("11", "00", None))),
        ("at 10pm", Ok(("10", "00", Some("pm")))),
        ("at 12:13 am", Ok(("12", "13", Some("am")))),
        ("13:42pm", Ok(("13", "42", Some("pm")))),
        ("15:30", Ok(("15", "30", None))),
        ("at 5", Ok(("5", "00", None))),
        ("32:412", Ok(("32", "412", None))),
        ("at 32:281am", Ok(("32", "281", Some("am")))),
        ("at 32pm", Ok(("32", "00", Some("pm")))),
        ("night time", Err(())),
        ("at night", Err(())),
    ];

    for test_case in test_times {
        let result = parse_time(test_case.0);
        if test_case.1.is_ok() {
            let (_, actual) = result.unwrap();
            assert_eq!(actual, test_case.1.unwrap());
        } else {
            assert!(result.is_err());
        }
    }
}

fn parse_date(input: &str) -> IResult<&str, ReminderDate> {
    let (input, (_, opt_repeat, _, date)) = tuple((
        multispace0,
        opt(alt((value(true, tag("every")), value(false, tag("on"))))),
        multispace0,
        rest
    ))
    .parse(input)?;
    let repeat = opt_repeat.unwrap_or(false);
    Ok((input, (date, repeat)))
}

#[test]
fn test_parse_date() {
    let test_cases: [(&str, Result<(&str, bool), ()>); 7] = [
        (" every Sunday", Ok(("Sunday", true))),
        ("every Monday", Ok(("Monday", true))),
        ("on Tuesday ", Ok(("Tuesday ", false))),
        ("tomorrow", Ok(("tomorrow", false))),
        ("today", Ok(("today", false))),
        ("on 08/25", Ok(("08/25", false))),
        ("every 3rd", Ok(("3rd", true))),
    ];

    for test_case in test_cases {
        let result = parse_date(test_case.0);
        if test_case.1.is_ok() {
            let (_, actual) = result.unwrap();
            assert_eq!(actual, test_case.1.unwrap());
        } else {
            assert!(result.is_err());
        }
    }
}

fn parse_task(input: &str) -> IResult<&str, Reminder> {
    let (input, (vtask, (time, date))) =
        many_till(anychar, pair(parse_time, parse_date)).parse(input)?;
    let text = vtask
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");
    Ok((input, Reminder { text, time, date }))
}

fn main() {
    let test_cases = [
        "go feed the fish at 10am",
        "feed the fish at 10:00am",
        "walk the dog 10:00am today",
        "feed the cat at 4 tomorrow",
        "get haircut at 14:24 pm",
        "credit card pay at 8am",
        "credit card pay at 8:00 every 20th",
        "cafe with Omar at Borrone 17:30",
        "cafe with Justin at Ginza at 6 on 08/23",
        "pick up books at library at 10am every Sunday",
    ];

    for input in test_cases {
        println!("> {}\n= {:?}\n", input, parse_task(input).unwrap().1);
    }
}
