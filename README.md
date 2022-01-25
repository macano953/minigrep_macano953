# minigrep

## Installation

Ensure [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) is installed.

Clone the repo:

```bash
git clone https://github.com/macano953/minigrep_macano953
```

`cd` into the cloned directory and run:

```bash
cargo install --path .
```

NOTE: This will install the binary in `$HOME/.cargo/bin` by default

## Usage

`minigrep` takes two arguments: a query (the text to search) and the filename in which to look for the query. If the file contains the query provided, minigrep will print out the entire matching line.

Example:

```bash
minigrep "some text to search" some-file.txt
```

By default, the search text will be case sensitive. To enable case insensitive search, set the `CASE_INSENSITIVE` environment variable. Example:

```bash
CASE_INSENSITIVE=1 minigrep "TO" poem.txt
```