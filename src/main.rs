//! git-leaf is a command line utility to create git branch with a defined format
//!
//! Project Repository: (https://gitlab.com/jveillet/git-leaf)[gitlab.com/jveillet/git-leaf]
//!
//! # Licence
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! # Usage
//! ```
//! git-leaf 1.0.0
//! Jérémie Veillet <jeremie.veillet@gmail.com>
//! Git plugin to automatically name branches based on a convention.
//!
//! USAGE:
//!    git-leaf --issue <NAME> --title <TITLE>
//!
//! FLAGS:
//!    -h, --help       Prints help information
//!    -V, --version    Prints version information

//! OPTIONS:
//!    -i, --issue <NAME>     Issue name, ex: JIRA-1234
//!   -t, --title <TITLE>    Issue title
//! ```
//!
extern crate clap;
extern crate regex;

use clap::{App, Arg};

mod git;

fn main() {
    let matches = App::new("git-leaf")
        .version("1.0.0")
        .author("Jérémie Veillet <jeremie.veillet@gmail.com>")
        .about("Git plugin to automatically name branches based on a convention.")
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

    if git::is_present() {
        // Get the branch name and title from the command line arguments
        let name = matches.value_of("issue").unwrap();
        let title = git::format_title(&matches.value_of("title").unwrap());
        // Format the branch name (issue number + issue title)
        let branch_name = format!("{}-{}", name, title);
        // Call the git command line tool to create the branch with this name
        git::create_branch(&branch_name);
    }
}
