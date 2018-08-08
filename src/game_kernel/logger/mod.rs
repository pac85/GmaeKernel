extern crate colored;
use self::colored::*;

enum Message<'a>
{
    Message(&'a str),
    Warning(&'a str),
    Error  (&'a str),
}

fn log(msg: Message)
{
    match msg
    {
        Message::Message(s) => {println!("[  ok   ] {}", s);},
        Message::Warning(s) => {println!("[  {}   ] {}","warn".yellow() ,s);},
        Message::Error(s)   => {println!("[  {}   ] {}","err".red() ,s);},
    }
}

pub fn log_msg(s: &str)
{
    log(Message::Message(s));
}

pub fn log_warn(s: &str)
{
    log(Message::Warning(s));
}

pub fn log_err(s: &str)
{
    log(Message::Error(s));
}
