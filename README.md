# Base64 in Rust 
my implementation of base64 in rust.   



# How to use
```rust
mod base64;

fn main() {

    let mut input_str: String = String::new();
    std::io::stdin().read_line(&mut input_str).unwrap();
    input_str.pop();

    let encoded = base64::encode_base64(input_str);
    let decoded = base64::decode_base64(encoded.clone());
    print!("{encoded} {decoded}\n",);
}
```


# Sources
[Implementing Base64 from scratch in Rust](https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1)