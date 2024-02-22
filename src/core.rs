use std::ffi::OsString;

use clap::{Parser, ValueEnum};

pub fn core() {
    let args = Args::parse();
    let mut method = args.method;
    println!("{:?}", method);
    println!("{:?}", args.url);
    println!("{:?}", args.body);
}

#[derive(Parser, Debug)]
#[clap(name = "http")]
struct Args {
    #[arg(help = "The URL to send the request to")]
    url: OsString,

    #[arg(default_value = "GET", ignore_case = true)]
    method: Method,

    #[arg()]
    body: Vec<OsString>,
}

#[derive(Debug, Clone, ValueEnum, Copy)]
enum Method {
    #[value(name = "BLANK")]
    BLANK,
    #[value(name = "GET")]
    GET,
    #[value(name = "POST")]
    POST,
    // PUT,
    // DELETE,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
