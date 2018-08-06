# git-leaf

git-leaf is a CLI tool to create branch named under a convention written in Rust

## Usage

`$ git-leaf -h`

```bash
git-leaf 0.1.0
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