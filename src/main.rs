//! git-leaf is a command line utility to create git branch with a defined format
//!
//! Project Repository: (https://github.com/jveillet/git-leaf)[github.com/jveillet/git-leaf]
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
//! git-leaf 1.1.0
//! Jérémie Veillet <jeremie.veillet@gmail.com>
//! Git plugin to automatically name branches based on a convention.
//!
//! USAGE:
//!    git-leaf --issue <NAME> --title <TITLE>
//!
//! OPTIONS:
//!   -h, --help             Print help information
//!   -i, --issue <NAME>     Issue name, ex: JIRA-1234
//!   -t, --title <TITLE>    Issue title
//!   -V, --version          Print version information
//! ```
//!
extern crate clap;
extern crate regex;

use clap::Parser;

mod git;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, required = true, help = "Issue name, ex: JIRA-1234")]
    issue: Option<String>,
    #[clap(short, long, required = true, help = "Issue title / description")]
    title: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if git::is_present() {
        // Get the branch name and title from the command line arguments
        let name = cli.issue.unwrap();
        let title = git::format_title(&cli.title.unwrap());
        // Format the branch name (issue number + issue title)
        let branch_name = format!("{}-{}", name, title);
        // Call the git command line tool to create the branch with this name
        git::create_branch(&branch_name);
    }
}
