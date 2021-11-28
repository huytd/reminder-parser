use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1, multispace0};
use nom::combinator::{opt, rest, value};
use nom::IResult;
use nom::Parser;
use nom::error::Error;
use nom::{
    multi::many_till,
    sequence::{pair, tuple},
};
use std::str;

#[derive(Debug)]
struct ReminderDate<'a> {
    content: &'a str,
    repeated: bool
}

#[derive(Debug)]
struct ReminderTime<'a> {
    hour: &'a str,
    minute: &'a str,
    meridiem: bool
}

#[derive(Debug)]
struct ReminderEvent<'a> {
    text: String,
    date: ReminderDate<'a>,
    time: ReminderTime<'a>
}

fn parse_time(input: &str) -> IResult<&str, ReminderTime> {
    let (remain, (_, _, hour, opt_min, _, am)) = tuple((
        opt(tag("at")),
        multispace0,
        digit1,
        opt(tuple((tag(":"), digit1))),
        multispace0,
        opt(alt((tag("am"), tag("pm")))),
    ))
    .parse(input)?;

    let (_, minute) = opt_min.unwrap_or(("", "00"));
    let meridiem = am == Some("am") || am == None;

    Ok((remain, ReminderTime { hour, minute, meridiem }))
}

#[test]
fn test_parse_time() {
    let test_times = [
        ("at 11:00", Ok(("11", "00", true))),
        ("at 10pm", Ok(("10", "00", false))),
        ("at 12:13 am", Ok(("12", "13", true))),
        ("13:42pm", Ok(("13", "42", false))),
        ("15:30", Ok(("15", "30", true))),
        ("at 5", Ok(("5", "00", true))),
        ("32:412", Ok(("32", "412", true))),
        ("at 32:281am", Ok(("32", "281", true))),
        ("at 32pm", Ok(("32", "00", false))),
        ("night time", Err(())),
        ("at night", Err(())),
    ];

    for test_case in test_times {
        let result = parse_time(test_case.0);
        if test_case.1.is_ok() {
            let (_, actual) = result.unwrap();
            let expected = test_case.1.unwrap();
            assert_eq!(actual.hour, expected.0);
            assert_eq!(actual.minute, expected.1);
            assert_eq!(actual.meridiem, expected.2);
        } else {
            assert!(result.is_err());
        }
    }
}

fn parse_date(input: &str) -> IResult<&str, ReminderDate> {
    let (remain, (_, opt_repeat, _, date)) = tuple((
        multispace0,
        opt(alt((value(true, tag("every")), value(false, tag("on"))))),
        multispace0,
        rest
    ))
    .parse(input)?;

    let repeated = opt_repeat.unwrap_or(false);
    let content = if date.trim().is_empty() { "today" } else { date.trim() };

    Ok((remain, ReminderDate { content, repeated }))
}

#[test]
fn test_parse_date() {
    let test_cases: [(&str, Result<(&str, bool), ()>); 8] = [
        (" every Sunday", Ok(("Sunday", true))),
        ("every Monday", Ok(("Monday", true))),
        ("on Tuesday ", Ok(("Tuesday", false))),
        ("tomorrow", Ok(("tomorrow", false))),
        ("today", Ok(("today", false))),
        ("on 08/25", Ok(("08/25", false))),
        ("every 3rd", Ok(("3rd", true))),
        ("", Ok(("today", false)))
    ];

    for test_case in test_cases {
        let result = parse_date(test_case.0);
        if test_case.1.is_ok() {
            let (_, actual) = result.unwrap();
            let expected = test_case.1.unwrap();
            assert_eq!(actual.content, expected.0);
            assert_eq!(actual.repeated, expected.1);
        } else {
            assert!(result.is_err());
        }
    }
}

fn parse_event(input: &str) -> IResult<&str, ReminderEvent> {
    let (input, (vtask, (time, date))) =
        many_till(anychar, pair(parse_time, parse_date)).parse(input)?;
    let text = vtask
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .to_string();
    Ok((input, ReminderEvent { text, time, date }))
}

#[test]
fn test_parse_event() {
    let test_events: [(&str, Result<(&str, &str, &str, bool, &str, bool), ()>); 10] = [
        ("go feed the fish at 10am", Ok(("go feed the fish", "10", "00", true, "today", false))),
        ("feed the fish at 10:00am", Ok(("feed the fish", "10", "00", true, "today", false))),
        ("walk the dog 10:00am today", Ok(("walk the dog", "10", "00", true, "today", false))),
        ("feed the cat at 4 tomorrow", Ok(("feed the cat", "4", "00", true, "tomorrow", false))),
        ("get haircut at 14:24 pm", Ok(("get haircut", "14", "24", false, "today", false))),
        ("credit card pay at 8am", Ok(("credit card pay", "8", "00", true, "today", false))),
        ("credit card pay at 8:00 every 20th", Ok(("credit card pay", "8", "00", true, "20th", true))),
        ("cafe with Justin at Ginza at 6 on 08/23", Ok(("cafe with Justin at Ginza", "6", "00", true, "08/23", false))),
        ("pick up books at library at 10am every Sunday", Ok(("pick up books at library", "10", "00", true, "Sunday", true))),
        ("lorem ipsum doro tata", Err(()))
    ];

    for test_case in test_events {
        let result = parse_event(test_case.0);
        if test_case.1.is_ok() {
            let (_, actual) = result.unwrap();
            let expected = test_case.1.unwrap();
            assert_eq!(actual.text, expected.0);
            assert_eq!(actual.time.hour, expected.1);
            assert_eq!(actual.time.minute, expected.2);
            assert_eq!(actual.time.meridiem, expected.3);
            assert_eq!(actual.date.content, expected.4);
            assert_eq!(actual.date.repeated, expected.5);
        } else {
            assert!(result.is_err());
        }
    }
}

fn main() {
    let event = parse_event("write new blog post at 9am every 14/12").unwrap();
    println!("{:?}", event);
}
