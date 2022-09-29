#![allow(clippy::use_self)]

use clap::Parser;
use logos::Logos;


#[derive(Logos, Debug, PartialEq)]
enum AbcToken {
    // Tokens can be literal strings, of any length.
    #[regex("a*b*c*")]
    Match,

    #[error]
    NoMatch,
}

#[derive(Logos, Debug, PartialEq)]
enum ZeroOneOneToken {
    // Tokens can be literal strings, of any length.
    #[regex("1*(0+1)*0*")]
    Match,

    #[error]
    NoMatch,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(value_parser)]
    file: String,
}

fn matchAbc(line: &str) {
    let mut lex = AbcToken::lexer(line);
    print!("{} ", line);
    match lex.next() {
        Some(AbcToken::Match) if lex.next().is_none() => print!("YES"),
        _ => print!("NO"),
    }

    println!();
}

fn matchZeroOneOne(line: &str) {
    let mut lex = ZeroOneOneToken::lexer(line);
    print!("{} ", line);
    let next = dbg!(lex.next());
    dbg!(lex.span());
    match next {
        Some(ZeroOneOneToken::Match) if lex.next().is_none() => print!("YES"),
        _ => print!("NO"),
    }

    println!();
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    for line in std::fs::read_to_string(args.file)?.lines() {
        // matchAbc(line);
        matchZeroOneOne(line);
    }

    Ok(())
}
