# fhtml

[<img alt="github" src="https://img.shields.io/badge/github-jonahlund/fhtml-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/jonahlund/fhtml)
[<img alt="crates.io" src="https://img.shields.io/crates/v/fhtml.svg?style=for-the-badge&logo=rust" height="20">](https://crates.io/crates/fhtml)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-fhtml-66c2a5?style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/fhtml)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/jonahlund/fhtml/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/jonahlund/fhtml/actions?query=branch%3Amain)

fhtml provides convenient macros to write formatted HTML in Rust with embedded
expressions.

<br>

## Components

Components can be written in a number of ways, but the common ways to create
reusable components include:

- **Function components** — A function component can be a simple function that
    accepts some arguments and returns the computed HTML. These are the most
    common types of components across web frameworks. For fhtml, function
    components are not always the best way to create a component, since unlike
    most JSX frameworks, function components do not get special treatment when
    used inside an fhtml macro.

- **Struct components** — A struct that implements [`Display`] is arguably the
    most JSX-like way to use components, since you can specify fields or "props"
    in an arbitrary order, and use convenient traits like `Default`.

- **Macros** — In Rust 1.71, flattening of nested [`format_args!`] was
    introduced, but this only works if macros are invoked, not functions nor
    methods, even if they are inlined. So for smaller components, using a macro
    that returns [`fhtml::format_args!`] is the most efficient kind of
    component, since they are usually zero-cost.

[`Display`]: https://doc.rust-lang.org/stable/std/fmt/trait.Display.html
[`format_args!`]: https://doc.rust-lang.org/stable/std/macro.format_args.html
[`fhtml::format_args!`]: https://docs.rs/fhtml/latest/fhtml/macro.format_args.html

<br>

## Nested formatting

You often need to do additional formatting inside your HTML, and you might be
tempted to use the standard [`format!`] for that. However, this is not the most
efficient way of doing additional formatting. Instead, [`format_args!`] should
be used in most cases. The overhead of [`format_args!`] is typically zero-cost
since nested [`format_args!`] calls are flattened by the compiler.

[`format!`]: https://doc.rust-lang.org/stable/std/macro.format.html
[`format_args!`]: https://doc.rust-lang.org/stable/std/macro.format_args.html

```rust
let puppy_kind = "Golden Retriever";
fhtml::format! {
    <img
        alt={format_args!("A happy {} playing", puppy_kind)}
        src="puppy.jpg"
    />
}
// Is equivalent to writing:
std::format!("<img alt=\"A happy {} playing\" src=\"puppy.jpg\">", puppy_kind)
```

<br>
    
## Const formatting

There are often situations when you want to write some HTML without using any
runtime values or variables. For this, you can use [`fhtml::concat!`].

```rust
const MY_PAGE: &str = fhtml::concat! {
    <!DOCTYPE html>
    <head>
        <title>"My HTML Page"</title>
    </head>
    <body>
        <h1>"Welcome to my HTML page!"</h1>
        {include_str!("../my-page.html")}
    </body>
}
```

[`fhtml::concat!`]: https://docs.rs/fhtml/latest/fhtml/macro.concat.html

<br>

## Escaping

Values are not escaped automatically. fhtml exports a simple escape function.
For more complex escaping, [html-escape](https://crates.io/crates/html-escape)
may be sufficient.

#### License

<sup>
Licensed under <a href="LICENSE">MIT license</a>.
</sup>
