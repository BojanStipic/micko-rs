# micko-rs

miniC compiler implemented in Rust.

[![CI](https://github.com/BojanStipic/micko-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/BojanStipic/micko-rs/actions/workflows/ci.yml)

## About

micko-rs is written in Rust and uses parser combinator library [Chumsky](https://github.com/zesterer/chumsky).
Parser combinators are a technique for implementing parsers by defining them in
terms of other parsers.
The resulting parsers use a [recursive descent](https://en.wikipedia.org/wiki/Recursive_descent_parser)
strategy to transform a stream of tokens into an output.
Using parser combinators to define parsers is roughly analagous to using Rust's
[Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait to
define iterative algorithms: the type-driven API of Iterator makes it more
difficult to make mistakes and easier to encode complicated iteration logic
than if one were to write the same code by hand.
The same is true of parser combinators.

miniC is a programming language created for educational purposes.
It is a strict subset of the C programming language.
miniC is used at the University of Novi Sad, Faculty of Technical Sciences,
to teach the Compilers course.

## Installation options

### Download precompiled binaries

Precompiled binaries are available on [Releases](https://github.com/BojanStipic/micko-rs/releases) page.
Only binaries for `x86_64-unknown-linux-gnu` target are provided.

### Compiling from source

#### Prerequisites

* [Rust language toolchain](https://www.rust-lang.org/tools/install)

#### Compile and install using Cargo:

```bash
cargo build --release
cargo install --path .
```

## Usage

```
USAGE:
    micko-rs <INPUT>

ARGS:
    <INPUT>    miniC file path

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## License

    Copyright (C) 2022 Bojan Stipic

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
