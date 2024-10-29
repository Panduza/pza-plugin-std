# pza-plugin-std



```bash

# Install once the tool
cargo install cargo-post

# Then build with post build operations
cargo post build --features log

```

## std.serial-port

```json
{
    "devices": [
        {
            "name": "pico",
            "dref": "std.serial-port"
        }
    ]
}
```