extern crate clap;

use clap::{App, Arg};
use std::process::Command;

fn main() {
    let matches = App::new("git-leaf")
        .version("0.2.0")
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

    if is_git_present() {
        let name = matches.value_of("issue").unwrap();
        let title = create_issue_title(&matches.value_of("title").unwrap());

        let branch_name = format!("{}-{}", name, title);

        // Call the git command line tool to create the branch with this name
        git_create_branch(&branch_name);
    }
}

/// Create the issue title
///
/// # Arguments
///
/// * `full_title` - A reference to a string containing the title sentence (with whitespaces)
///
/// # Result
///
/// * `String` - String containing the formatted title
/// - Underscores instead of whitespaces
/// - Only the first word is capitalized
/// - All the other words in the title are lowercase
///
/// # Examples
///
/// let title = create_issue_title("My Awesome Title");
/// println!("{}", title);
/// => My_awesome_title
///
fn create_issue_title(full_title: &str) -> String {
    // Replace all whitespaces with underscores
    let underscores = str::replace(full_title, " ", "_");
    // Get separately the first letter of the title and the rest of the sentence
    let first_word = &underscores[0..1];
    let other_words = &underscores[1..underscores.len()];
    // Combine the first letter + the rest of the sentence in lowercase
    format!("{}{}", first_word, other_words.to_lowercase())
}

/// Create a named git branch
///
/// # Arguments
///
/// * `branch_name` - A reference to a strign containing the full branch name that need to be created
///
/// # Panics
///
/// Panics if the git command line tool failes to do a checkout
///
fn git_create_branch(branch_name: &str) {
    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch_name)
        .output()
        .expect("the git checkout command failed");
}

/// Test if the git comand line tool is present on the system
///
/// # Result
///
/// * `bool` - true if the command succeeded
///
fn is_git_present() -> bool {
    let output = Command::new("git")
        .arg("--version")
        .output()
        .expect("git was not found on the system");

    output.status.success()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_git_present() {
        assert_eq!(is_git_present(), true);
    }

    #[test]
    fn should_format_title() {
        let sentence = String::from("My Awesome Issue Title");
        let title = create_issue_title(&sentence);
        assert_eq!(title, "My_awesome_issue_title");
    }
}
