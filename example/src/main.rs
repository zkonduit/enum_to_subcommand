use std::fmt::{Formatter, Result};
use tosubcommand::{ToFlags, ToSubcommand};

#[derive(Debug)]
pub enum Modes {
    One,
    Two,
    Three,
}

#[derive(Debug, ToFlags)]
pub struct SubStruct {
    a: i32,
    b: i32,
    c: i32,
    d: Option<i32>,
    e: Option<i32>,
}

impl std::fmt::Display for Modes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                Modes::One => "one",
                Modes::Two => "two",
                Modes::Three => "three",
            }
        )
    }
}

impl ToFlags for Modes {
    /// Convert the struct to a subcommand string
    fn to_flags(&self) -> Vec<String> {
        vec![format!("{}", self)]
    }
}

#[derive(ToSubcommand)]
enum Cmd {
    Mock {
        with: String,
        without: String,
        path: std::path::PathBuf,
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
        vec: Vec<i32>,
        opt: Option<i128>,
        opt2: Option<i128>,
        sub: SubStruct,
    },
    GenSomething {
        something_to_gen: usize,
        tuple: (i32, i32),
        variables: Vec<(i32, i32)>,
    },
}

fn main() {
    let mock = Cmd::Mock {
        with: "with".to_string(),
        without: "without".to_string(),
        path: std::path::PathBuf::from("foo/bar/file.text"),
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
        vec: vec![1, 2, 3],
        opt: Some(1),
        opt2: None,
        sub: SubStruct {
            a: 0,
            b: 1,
            c: 2,
            d: None,
            e: Some(3),
        },
    };
    let gen_something = Cmd::GenSomething {
        something_to_gen: 2,
        tuple: (1, 2),
        variables: vec![(1, 2), (3, 4)],
    };

    println!("{}", mock.to_subcommand().join(" "));
    println!("{}", empty.to_subcommand().join(" "));
    println!("{}", run.to_subcommand().join(" "));
    println!("{}", long.to_subcommand().join(" "));
    println!("{}", nested.to_subcommand().join(" "));
    println!("{}", gen_something.to_subcommand().join(" "));

    assert_eq!(
        mock.to_subcommand().join(" "),
        "mock --with with --without without --path foo/bar/file.text"
    );
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
        "nested --mode one --level 1 --vec 1,2,3 --opt 1 --a 0 --b 1 --c 2 --e 3"
    );
    assert_eq!(
        gen_something.to_subcommand().join(" "),
        "gen-something --something-to-gen 2 --tuple 1->2 --variables 1->2,3->4"
    )
}
