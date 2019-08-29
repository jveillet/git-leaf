# git-leaf

git-leaf is a command line utility for creating git branches using a convention, written in Rust.
It can be used as a Git plugin for convenience.

This project is a Rust playground, it might not be helpful to you, it's an OSS project so you might tweak it as you want.

## Description

git-leaf is an executable that takes an issue name, could be a JIRA ticket number, a Github issue number,..
And a title, which is a simple string sentence, representing your issue title.

The output will be a git branch with a name formated with the concatenation of the issue name + title.
The whitespace will be replaced by underscores, and only the first letter of the sentence will be capitalized.

## Prerequisites

You will need a version of the Rust Programming language (>= 1.28.0), it should come with Cargo, the Rust packet manager.
See the [Rust documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html) for more details.

This project is known to work on On linux Debian 9, MacOS High Sierra and Mojave.

Feel free to test it on another OS and open issues if needed.

### Dependencies

You will need a working version of git installed on your system.
Note that it is already provided in the Dockerfile.

## Installing

```bash
$ git clone git@gitlab.com:jveillet/git-leaf.git
$ cd git-leaf
$ cargo install --path .
```

This will install the binary in the `~/.cargo/bin`.

You can add this path to your OS path `$PATH` in your `.bashrc` or `.zshrc`:
```
export PATH="$PATH:$HOME/bin:$HOME/.local/bin:$HOME/.cargo/bin"

```
By doing this, you will also get `git` integration for free, neat! :)

```
$ git leaf -h
```

## Compile from source

**Note**: You can also build this project using Docker (see the Compile with Docker section).

### Compile locally

```bash
$ git clone git@gitlab.com:jveillet/git-leaf.git
$ cd git-leaf
$ cargo build --release
```

### Compile with Docker

```bash
$ git clone git@gitlab.com:jveillet/git-leaf.git
$ cd git-leaf
$ docker-compose build
$ docker-compose run --rm app cargo build --release
```

## Usage

`$ git-leaf -h` or as a git plugin `$ git leaf -h`

```bash
git-leaf 1.0.0
Jérémie Veillet <jeremie.veillet@gmail.com>
Git plugin to automatically name branches based on a convention.

USAGE:
    git-leaf --issue <NAME> --title <TITLE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --issue <NAME>     Issue name, ex: JIRA-1234
    -t, --title <TITLE>    Issue title
```

## Example

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

## Contributing

You want to contribute to the Project? Yeah!! :v: 🎉  Contributors are always welcome! :thumbsup:

**Note**: One of the best ways to help right now is to use the utility and report issues!

### Bugs

If you find bugs, first go to the [issues page](https://gitlab.com/jveillet/git-leaf/issues) and search if a related issue isn't listed there.

Create a new issue and insert any informations that can help to reproduce the observed behavior:
* Command context
* Stack trace
* Expected bahevior
* Current behavior
* OS / environment

Consider adding the ~bug label on your ticket.

### Feature requests

Create a new issue on the [issues page](https://gitlab.com/jveillet/git-leaf/issues) and add a clear description of what the new feature should look like.

Consider adding the ~"feature request" label on your ticket.

### Merge Requests

1. Fork git-leaf
2. Clone your fork `git clone https://gitlab.com/$YOUR_USERNAME/git-leaf && cd git-leaf`
3. Create new branch `git checkout -b new-branch`
4. Make your changes, and commit `git commit -am "your message"`

## Licence.

git-leaf is a free software: you can redistribute it and/or modify it under the terms of the [GNU GPL v3](LICENCE).

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see http://www.gnu.org/licenses/.

