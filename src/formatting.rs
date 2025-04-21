use std::fmt;

#[allow(dead_code)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[allow(dead_code)]
pub enum Style {
    Bold,
    Underline,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color_code = match *self {
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
        };
        write!(f, "{}", color_code)
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let style_code = match *self {
            Style::Bold => "1",
            Style::Underline => "4",
        };
        write!(f, "{}", style_code)
    }
}

pub struct Formatter;

impl Formatter {
    pub fn apply_formatting(text: String, color: Color, style: Option<Style>) -> String {
        let style_code = style.map(|s| format!("{}", s)).unwrap_or_else(String::new);
        format!("\x1b[{};{}m{}\x1b[0m", style_code, color, text)
    }
}

pub mod builtins {
    // use super::*;
    // uncomment if you need to use the Color and Formatter structs

    pub fn echo(args: &[&str]) {
        for arg in args {
            print!("{}", arg);
            if !arg.ends_with('\n') {
                print!(" ");
            }
        }
        println!();
    }
}
