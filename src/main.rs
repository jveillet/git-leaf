extern crate clap;

use clap::{App, Arg};
use std::process::Command;

fn main() {
    let matches = App::new("git-leaf")
        .version("0.1.0")
        .author("Jérémie Veillet <jeremie.veillet@gmail.com>")
        .about("CLI to automatically name git branches based on a convention.")
        .arg(Arg::with_name("issue")
            .short("i")
            .long("issue")
            .value_name("NAME")
            .help("Issue name, ex: JIRA-1234")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("title")
            .short("t")
            .long("title")
            .value_name("TITLE")
            .help("Issue title")
            .takes_value(true)
            .required(true)
        )
        .get_matches();

    let mut issue_name: String = String::new();
    let mut issue_title: String = String::new();

    if let Some(issue) = matches.value_of("issue") {
        issue_name = issue.to_string();
    }

    if let Some(title) = matches.value_of("title") {
        // Replace all whitespaces with underscores
        let underscores = str::replace(title, " ", "_");
        // Get separately the first letter of the title and the rest of the sentence
        let first_word = &underscores[0..1];
        let other_words = &underscores[1..underscores.len()];
        // Combine the first letter + the rest of the sentence in lowercase
        let complete_title = format!("{}{}", first_word, other_words.to_lowercase());
        issue_title = complete_title;
    }

    let branch_name = format!("{}-{}", issue_name, issue_title);

    // Call the git command line tool to create the branch with this name
    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch_name)
        .output()
        .expect("git checkout command failed to start");
}
