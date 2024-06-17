# fhtml

Std-compatible HTML formatting macros

## Overview

`fhtml` Provides formatting macros for writing HTML without the annoyance of dealing with HTML inside string literals. A few highlights:

- **simplicity:** no complex templating syntax, just plain HTML with embedded expressions and format specifiers
- **compatibility:** since `fhtml` is simply a wrapper over `std` macros, meaning that you can easily use idiomatic Rust, such as implementing `fmt::Display` or `fmt::Write` for creating components, or integrate with existing libraries and tools

## Syntax

- HTML is typed as-is, unquoted:
```rust
fhtml::format!(<input />);
```
- Expressions are passed in using braces:
```rust
fhtml::format!(<div>{1 + 1}</div>);
```
- Text nodes are quoted:
```rust
fhtml::format!(<p>"Some text"</p>);
```
- Format specifiers are written after expressions:
```rust
fhtml::format!(<code>{vec![1, 2, 3]:?}</code>);
```

## Usage

### Writing to a buffer

```rust
let mut buffer = String::new();
fhtml::write!(buffer, <div>"Hello, World!"</div>);
```

### Components

#### Function components

```rust
fn heading(label: &'static str) -> String {
    fhtml::format! {
        <h1>{label}</h1>
    }    
}

let page = fhtml::format! {
    <main>
        {heading("My Heading")}
        <div>"My Content"</div>
    </main>
};
```

#### Struct components

```rust
use std::fmt;

struct Product {
    name: &'static str,
    price: f32,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fhtml::write!(f,
            <article>
                <h2>{self.name}</h2>
                <h3>"$" {self.price}</h3>
            </article>
        )
    }
}

let products = fhtml::format! {
    <h1>"Our products"</h1>
    {Product {
      name: "Arabica Coffee Beans",
      price: 3.99
    }}
    {Product {
      name: "Sourdough Bread",
      price: 2.49
    }}
};
```

### Format specifiers

```rust
fhtml::format!(<code>{vec![1, 2, 3]:?}</code>); // "<code>[1, 2, 3]</code>"
fhtml::format!(<span>{10:#b}</span>);           // "<span>0b1010</span>"
```

### Iterators

```rust
fhtml::format! {
    <ul>
        {
            (0..10).fold(String::new(), |mut output, i| {
                let _ = fhtml::write!(output,
                    <li>{i}</li>
                );
                output
            })
        }
    </ul>
}
```
