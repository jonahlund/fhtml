# fhtml

Fast and simple macros for formatting, identical to `write!` or `format!`, but tailored for HTML.

## Usage
```toml
[dependencies]
fhtml = "0.1"
```

### Simple formatting
```rust
let output = fhtml::format! { <div>"Hello, World!"</div> };

assert_eq!(output, "<div>Hello, World!</div>");
```

### Writing to a buffer
```rust
let mut output = String::new();

let _ = fhtml::write! { output, <div>"Hello, World!"</div> };

assert_eq!(output, "<div>Hello, World!</div>");
```

### Expressions
```rust
let output = fhtml::format! { <div>{1 + 2}</div> };

assert_eq!(output, "<div>3</div>");
```
Since these macros expand to `std::write!`, any values that can be used in `std::write!` can be used here.

### Components
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
  {Product { name: "Coffee".into(), price: 4.99 }}
  {Product { name: "Bread".into(), price: 3.99 }}
};

assert_eq!(products, "\
<h1>Our products</h1>\
<article><h2>Coffee</h2><h3>$4.99</h3></article>\
<article><h2>Bread</h2><h3>$3.99</h3></article>\
");
```
