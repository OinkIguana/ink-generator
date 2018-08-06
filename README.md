[Ink]: https://www.inklestudios.com/ink/
[generators]: https://github.com/rust-lang/rfcs/blob/master/text/2033-experimental-coroutines.md

# Ink Generator

The Ink Generator generates Rust code from an [Ink] script. The Ink is parsed, then transformed to
the corresponding Rust code by using lots of [generators], and a very lightweight runtime to wrap
those generators up in to a nice-ish, but safe API.

**This is a work in progress**. Use at your own risk.

## Note

As generators are unstable, and this project uses them heavily, it only works on the Nightly Rust
for now.

Also, this is also not a complete, nor accurate implementation of the Ink language, and likely never
will be. Many features are missing, and lots of validation is likely skipped. If you do not provide
a valid Ink script to the generator, it is currently *undefined behavior*.

Even valid Ink scripts are very likely going to have issues, given the custom parser implementation
and completely different runtime experience. If an Ink script you have written is not interpreted as
expected, please log an issue on Github.

## Features

As of right now, only the most basic and fundamental features are supported, and most of those are
not fully tested:

*   Regular lines
*   Choices (`*`)
*   Glue (`<>`)
*   Knots (`==`)
*   Stitches (`=`)
*   Diverts (`->`)

## Running instructions

This package contains two binaries: `inkgen` and `inkplay` (sort of).

### `inkgen`

`inkgen` takes two parameters: the input file (.ink) and the output file (.rs). The output file will
contain a Rust module with the same name as the output file. The output file both the input and
output files can be excluded. If a filename is missing, `stdin` or `stdout` is used instead,
respectively.

```
inkgen story.ink story.rs
```

The generated code comes out really ugly, so if you are planning to read it, it is suggested to use
`rustfmt` to format the output, as follows:

```
inkgen story.ink | rustfmt > story.rs
```

In this case, the module in the generated file will take the same name as the input file. If `stdin`
is used, the module will be called `story`, for lack of a better name.

### `inkplay`

`inkplay` provides a basic implementation of the Ink runtime. However, since it is Rust code being
generated, it requires compiling every time you change the story. To use the `inkplay` binary, you
will need to build and run it yourself. Use `inkgen` to generate the file `src/bin/test.rs`, then
just run `cargo run --bin inkplay`.

Note that `cargo build` will fail. I'm not sure why, but just use `run` always and it's ok!
