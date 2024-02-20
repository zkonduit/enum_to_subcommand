use tosubcommand::ToSubcommand;
use std::fmt::{Formatter, Result};

pub enum Modes {
    One, 
    Two,
    Three,
}

impl std::fmt::Display for Modes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match self {
            Modes::One => "one",
            Modes::Two => "two",
            Modes::Three => "three",
        })
    }
}


#[derive(ToSubcommand)]
enum Cmd {
    Mock {
        with: String,
        without: String,
    },
    Empty,
    Run {
        with: String,
        without: String,
        and: i128,
        or: bool,
    },
    Long {
        or_not: bool,
        and_not: i128,
        with_not: String,
        without_not: String,
    },
    Nested {
        mode: Modes,
        level: i32,
    }
}

fn main() {
    let mock = Cmd::Mock {
        with: "with".to_string(),
        without: "without".to_string(),
    };
    let empty = Cmd::Empty;
    let run = Cmd::Run {
        with: "with".to_string(),
        without: "without".to_string(),
        and: 1,
        or: true,
    };
    let long = Cmd::Long {
        or_not: true,
        and_not: 1,
        with_not: "with".to_string(),
        without_not: "without".to_string(),
    };
    let nested = Cmd::Nested {
        mode: Modes::One,
        level: 1,
    };

    println!("{}", mock.to_subcommand().join(" "));
    println!("{}", empty.to_subcommand().join(" "));
    println!("{}", run.to_subcommand().join(" "));
    println!("{}", long.to_subcommand().join(" "));
    println!("{}", nested.to_subcommand().join(" "));

    assert_eq!(mock.to_subcommand().join(" "), "mock --with with --without without");
    assert_eq!(empty.to_subcommand().join(" "), "empty");
    assert_eq!(
        run.to_subcommand().join(" "),
        "run --with with --without without --and 1 --or true"
    );
    assert_eq!(
        long.to_subcommand().join(" "),
        "long --or-not true --and-not 1 --with-not with --without-not without"
    );
    assert_eq!(
        nested.to_subcommand().join(" "),
        "nested --mode one --level 1"
    );
}
