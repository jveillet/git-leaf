extern crate clap;

use clap::{App, Arg};

mod git;
use git::git as cli;

fn main() {
    let matches = App::new("git-leaf")
        .version("0.2.1")
        .author("Jérémie Veillet <jeremie.veillet@gmail.com>")
        .about("CLI to automatically name git branches based on a convention.")
        .arg(
            Arg::with_name("issue")
                .short("i")
                .long("issue")
                .value_name("NAME")
                .help("Issue name, ex: JIRA-1234")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("title")
                .short("t")
                .long("title")
                .value_name("TITLE")
                .help("Issue title")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    if cli::is_present() {
        let name = matches.value_of("issue").unwrap();
        let title = cli::format_title(&matches.value_of("title").unwrap());

        let branch_name = format!("{}-{}", name, title);

        // Call the git command line tool to create the branch with this name
        cli::create_branch(&branch_name);
    }
}
