# fhtml

`fhtml` is a crate that provides fast and straightforward macros for generating HTML content, similar to standard `write!` and `format!` macros, but tailored specifically for HTML output.

## Installation

Add `fhtml` to your project by including it in your `Cargo.toml` file:

```toml
[dependencies]
fhtml = "0.2"
```

## Quick Start

### Basic Formatting

Generate simple HTML strings with `fhtml::format!`. This macro behaves similarly to `format!`, but for HTML:

```rust
let output = fhtml::format! { <div>"Hello, World!"</div> };
assert_eq!(output, "<div>Hello, World!</div>");
```

### Writing to a Buffer

Directly write HTML to a buffer using `fhtml::write!`:

```rust
let mut output = String::new();
let _ = fhtml::write! { output, <div>"Hello, World!"</div> };
assert_eq!(output, "<div>Hello, World!</div>");
```

### Incorporating Expressions

Embed expressions within HTML content:

```rust
let output = fhtml::format! { <div>{1 + 2}</div> };
assert_eq!(output, "<div>3</div>");
```

These macros expand to `std::write!`, supporting any compatible values.

### Custom Components via Display Trait

Create reusable HTML components by implementing `std::fmt::Display`:

```rust
use std::fmt;

struct Product {
    name: String,
    price: f32,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fhtml::write! { f,
            <article>
                <h2>{self.name}</h2>
                <h3>"$"{self.price}</h3>
            </article>
        }
    }
}

let products = fhtml::format! {
    <h1>"Our products"</h1>
    {Product {
      name: "Coffee".to_string(),
      price: 4.99
    }}
    {Product {
      name: "Bread".to_string(),
      price: 3.99
    }}
};

assert_eq!(products,
"<h1>Our products</h1>\
<article><h2>Coffee</h2><h3>$4.99</h3></article>\
<article><h2>Bread</h2><h3>$3.99</h3></article>");
```

### HTML Escaping

Note that `fhtml` does not perform HTML escaping implicitly. This means any HTML special characters included in strings will not be escaped automatically. To ensure your HTML content is secure from injection attacks, you can manually escape content using `fhtml::escape`:

```rust
let user_input = "<script>alert('xss');</script>";
let safe_output = fhtml::format! { <div>{fhtml::escape(user_input)}</div> };
assert_eq!(safe_output, "<div>&lt;script&gt;alert(&#39;xss&#39;);&lt;/script&gt;</div>");
```
