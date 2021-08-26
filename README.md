# Reminder Parser Experiment

An experiment parser that transform event input in natural language to a Rust data structure.

Written using Nom library.

## Grammar

Reminder input could be entered in the following form:

```
go feed the fish at 10pm every Thursday

or

feed the fish at 10:00 am
```

The structure of the event input could be broken into four parts:

```
<event-text> <time> <repeat> <date>
```

### Event Text
The `<event-text>` could be anything (alphanumerics, whitespaces,...).

### Time
Time is anything that goes after the keyword `"at"`, it could be either `at 10pm` or `11:32`. The keyword `"at"` could be omitted. Also, the periods `"am"` and `"pm"` is optional.

```
time = ?"at" + hour + ?(":" + minutes) + ?("am"|"pm")
```

By default, if no `minutes` is present, it should be returned as `00`.

### Date and Repeat
Date is anything that goes after the keyword `"on"` or `"every"`, either of them could be omitted. And if `"every"` is presented, it's the repeat indicator, that mean the event could be recurrence.

```
date = ?("on"|"every") + date
```

## Data Validation
To keep it simple, this parser does not handle data validation. So cases like this will also passed:

```
32:42 pm

24:59
```

This could be handled in later phase of the parser.

## Demo

```
> go feed the fish at 10am
= Reminder { text: "go feed the fish ", date: ("", false), time: ("10", "00", Some("am")) }

> feed the fish at 10:00am
= Reminder { text: "feed the fish ", date: ("", false), time: ("10", "00", Some("am")) }

> walk the dog 10:00am today
= Reminder { text: "walk the dog", date: ("today", false), time: ("10", "00", Some("am")) }

> feed the cat at 4 tomorrow
= Reminder { text: "feed the cat ", date: ("tomorrow", false), time: ("4", "00", None) }

> get haircut at 14:24 pm
= Reminder { text: "get haircut ", date: ("", false), time: ("14", "24", Some("pm")) }

> credit card pay at 8am
= Reminder { text: "credit card pay ", date: ("", false), time: ("8", "00", Some("am")) }

> credit card pay at 8:00 every 20th
= Reminder { text: "credit card pay ", date: ("20th", true), time: ("8", "00", None) }

> cafe with Omar at Borrone 17:30
= Reminder { text: "cafe with Omar at Borrone", date: ("", false), time: ("17", "30", None) }

> cafe with Justin at Ginza at 6 on 08/23
= Reminder { text: "cafe with Justin at Ginza ", date: ("08/23", false), time: ("6", "00", None) }

> pick up books at library at 10am every Sunday
= Reminder { text: "pick up books at library ", date: ("Sunday", true), time: ("10", "00", Some("am")) }
```
