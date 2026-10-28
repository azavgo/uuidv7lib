# Library to generate UUID version 7 RFC strings
## Supported formats: 
- Low letters case, e.g. 01a1821cadbe7a4cad2fd5f11dd95c64. Function signature: pub fn
  to_low() -> Arc<String>
- Capital letters case, e.g. 01A1821CADBE7A4CAD2FD5F11DD95C64. Function
  signature: pub fn to_capital() -> Arc<String>
- Low letters case with dashes, e.g. 01a1821c-adbe-7a4c-ad2f-d5f11dd95c64.
  Function signature pub fn to_low_dashes() -> Arc<String>
- Capital letters with dashes, e.g. 01A1821C-ADBE-7A4C-AD2F-D5F11DD95C64.
  Function signature pub fn to_capital_dashes() -> Arc<String>
  
## How to use this library:

1. Add to Cargo.toml: 
```Toml
    [dependencies]
    uuidv4 = {git = "https://github.com/azavgo/uuidv7lib", branch = "main"}
```
2. Generate version 7 UUID RFC string:

```Rust
    use uuidv7lib::*;

    fn main() {
        let uuidv7 = UUIDv7::new();
        println!("Generated UUID v7 RFC: {}", uuidv7.to_low_dashed()); 
    }
```


