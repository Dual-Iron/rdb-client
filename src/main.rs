// See documentation at https://rdb.dual-iron.xyz/

use console::{style, Key, Term};
use regex::Regex;
use reqwest::blocking::Response;
use std::io::{self, Write};

// Print immediately
macro_rules! printi {
    ($($arg:tt)*) => {
        print!($($arg)*);
        std::io::stdout().flush()?;
    };
}

fn user_prompt(
    term: &mut Term,
    mut allow_char: impl FnMut(&str, char) -> bool,
    mut allow_enter: impl FnMut(&str) -> bool,
) -> io::Result<String> {
    let mut buf = String::new();

    loop {
        match term.read_key()? {
            Key::Backspace => {
                if let Some(_) = buf.pop() {
                    term.clear_chars(1)?;
                }
            }
            Key::Char(c) => {
                if allow_char(&buf, c) {
                    buf.push(c);
                    term.write_fmt(format_args!("{c}"))?;
                }
            }
            Key::Enter => {
                if allow_enter(&buf) {
                    term.write_all(b"\n")?;
                    return Ok(buf);
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let mut term = Term::stdout();

    match run(&mut term) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("An error occurred while executing! {}", style(e).red())
        }
    }

    println!("\nPress any key to exit.");

    _ = term.read_key();
}

fn run(term: &mut Term) -> std::io::Result<()> {
    printi!("{}", style("secret      ").cyan());
    let secret = user_prompt(term, max_len(500), non_empty)?;
    printi!("{}", style("name        ").cyan());
    let name = user_prompt(term, name_and_owner_chars, non_empty)?;
    printi!("{}", style("owner       ").cyan());
    let owner = user_prompt(term, name_and_owner_chars, non_empty)?;
    printi!("{}", style("version     ").cyan());
    let version = user_prompt(term, max_len(50), valid_semver)?;
    printi!("{}", style("description ").cyan());
    let desc = user_prompt(term, max_len(500), |_| true)?;
    printi!("{}", style("icon        ").cyan());
    let icon = user_prompt(term, max_len(500), valid_url)?;
    printi!("{}", style("binary      ").cyan());
    let binary = user_prompt(term, max_len(500), valid_binary)?;
    printi!("{}", style("homepage    ").cyan());
    let homepage = user_prompt(term, max_len(500), valid_url_or_empty)?;

    printi!(
        "\nTo send this entry to rdb, press {}.\nTo cancel this operation, press {}. ",
        style("Enter").green(),
        style("Ctrl C").red()
    );

    while term.read_key()? != Key::Enter {}

    printi!("\n\nSending to rdb... ");

    let result = send_data(format!(
        r#"{{"secret":"{secret}","name":"{name}","owner":"{owner}","version":"{version}","description":"{desc}","icon":"{icon}","binary":"{binary}","homepage":"{homepage}"}}"#
    ));

    let err = style("error").red();

    match result {
        Err(e) => println!("There was an {err}: {e}"),
        Ok(response) => {
            let status = response.status();
            if status.is_server_error() {
                println!("The server encountered an {err}. Try again later.")
            } else if status.is_client_error() {
                println!(
                    "There was an {err}: {}",
                    response.text().unwrap_or_else(|_| status.to_string())
                )
            } else {
                println!("Successfully submitted {} to rdb!", style(name).green())
            }
        }
    }

    Ok(())
}

fn send_data(json: String) -> Result<Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    client
        .post("https://rdb.dual-iron.xyz/mods")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
}

fn name_and_owner_chars(s: &str, c: char) -> bool {
    s.len() + c.len_utf8() < 40 && (c.is_alphanumeric() || ['.', '-', '_'].contains(&c))
}

fn max_len(len: usize) -> impl Fn(&str, char) -> bool {
    move |s, c| s.len() + c.len_utf8() <= len
}

fn non_empty(s: &str) -> bool {
    !s.is_empty()
}

fn valid_url_or_empty(s: &str) -> bool {
    s.is_empty() || valid_url(s)
}

fn valid_url(s: &str) -> bool {
    if let Ok(u) = url::Url::parse(s) {
        u.scheme() == "https"
    } else {
        false
    }
}

fn valid_binary(s: &str) -> bool {
    lazy_static::lazy_static! {
        // Capture 1 = google drive ID
        static ref DRIVE: Regex = Regex::new(r#"https://drive.google.com/file/d/([^/]+)"#).unwrap();
        static ref DRIVE_DIRECT: Regex = Regex::new(r#"https://drive.google.com/uc\?export=download&id=[^/]+"#).unwrap();
        static ref GITHUB: Regex = Regex::new(r#"https://github.com/Dual-Iron/.+/.+/download/.+?/[^/]+"#).unwrap();
        static ref DISCORD: Regex = Regex::new(r#"https://cdn.discordapp.com/attachments/\d+/\d+/[^/]+"#).unwrap();
    }

    DRIVE.is_match(s) || DRIVE_DIRECT.is_match(s) || GITHUB.is_match(s) || DISCORD.is_match(s)
}

fn valid_semver(s: &str) -> bool {
    semver::Version::parse(s).is_ok()
}
