# git-leaf

git-leaf is a command line utility for creating git branches using a convention, written in Rust.

This project is a Rust playground, it might not be helpful to you, it's an OSS project so you might tweak it as you want.

# Description

git-leaf is an executable that takes an issue name, could be a JIRA ticket number, a Github issue number,..
And a title, which is a simple string sentence, representing your issue title.

The output will be a git branch with a name formated with the concatenation of the issue name + title.
The whitespace will be replaced by underscores, and only the first letter of the sentence will be capitalized.

### Prerequisites

You will need a version of the Rust Programming language (>= 1.28.0), it should come with Cargo, the Rust packet manager.
See the [Rust documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html) for more details.

This project has only been tested on Linux (Debian based distro). Feel free to test it on another OS and share the outcomes.

You will also need a working version of git installed on your system.

### Installing

```bash
$ git clone git@gitlab.com:jveillet/git-leaf.git
$ cd git-leaf
$ cargo install --bin git-leaf
```

### Compile from source

**Note**: You can also build this project using Docker (see the Compile with Docker section).

#### Compile locally

```bash
$ git clone git@gitlab.com:jveillet/git-leaf.git
$ cd git-leaf
$ cargo build --release
```

#### Compile with Docker

```bash
$ git clone git@gitlab.com:jveillet/git-leaf.git
$ cd git-leaf
$ docker-compose build
$ docker-compose run --rm app cargo build --release
```

## Usage

`$ git-leaf -h`

```bash
git-leaf 0.2.1
Jérémie Veillet <jeremie.veillet@gmail.com>
CLI to automatically name git branches based on a convention.

USAGE:
    git-leaf --issue <NAME> --title <TITLE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --issue <NAME>     Issue name, ex: JIRA-1234
    -t, --title <TITLE>    Issue title
```

### Example

```bash
$ cd my-awesome-project
> my-awesome-project -> master
$ git-leaf -i JIRA-1234 -t "My Awsome Issue Name"
> my-awesome-project -> JIRA-1234-My_awesome_project
```

## Tests

Running tests:

```bash
$ cargo test
```

With Docker:

```bash
$ docker-compose run --rm app cargo test
```

## Licence.

git-leaf is a free software: you can redistribute it and/or modify it under the terms of the [GNU GPL v3](LICENCE).

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see http://www.gnu.org/licenses/.