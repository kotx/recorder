# recorder

Simple record types for Rust

## Getting Started

Here's a simple example:

```rust
#[recorder::record] // this struct will be #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyRecord {
    a: String, // this will be `pub`
    b: u8, // this too!
    c: Vec<String>, // and this!
    #[record(skip)] d: String // this field will be preserved (private by default)
}
```
