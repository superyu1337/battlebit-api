# Battlebit-api [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/battlebit-api.svg
[crates.io]: https://crates.io/crates/battlebit-api

**A Battlebit API Library/Client for Rust**

---

What you may be looking for:
- [Examples](https://github.com/superyu1337/battlebit-api/tree/main/examples)
- [Documentation](https://docs.rs/battlebit-api)

## Example

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]
battlebit-api = "0.2.0"
```

</details>
<p></p>

```rust
use battlebit_api::BBApi;

fn main() {
    let api = BBApi::new();
    let server_list = api.server_list()
        .expect("Retrieving server list");

    server_list.into_iter().for_each(|server| {
        println!("{} [{}, {}, {}] ({}, {}, {})", 
            server.name(), 
            server.gamemode(), 
            server.map(),
            server.map_size(),
            server.hz(), 
            server.anti_cheat(),
            server.build()
        )
    });
}
```