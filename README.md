# pza-plugin-std



```bash

# Install once the tool
cargo install cargo-post

# Then build with post build operations
cargo post build --features plugin

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

## Tests

First start a platform with this configuration

```json
{
    "devices": [
        {
            "name": "serial plugin",
            "dref": "std.serial_stream",
            "settings": {
                "transport": "tcp",
                "tcp_ip": "127.0.0.1",
                "tcp_port": "12345",
                "serial_port_name": "COM5",
                "serial_baud_rate": "115200",
                "message_on_connect": false
            }
        }
    ]
}
```

Then start the tcp-server-test with this configuration
( the tcp-server-test is provided in the tools directory )

```json
{
    "ip": "127.0.0.1",
    "port": 12345
}
```

Then run

```bash
# Run basics tests for plugin_std
cargo test --test plugin_std
```
## Connection to the Zybo board 

Turn on the the Zybo board

start the TFTP server on the application provided by the project framework-on-SoC

Start a platform with this configuration

```json
{
    "devices": [
        {
            "name": "serial plugin",
            "dref": "std.serial_stream",
            "settings": {
                "transport": "tcp",
                "tcp_ip": "192.168.35.11",
                "tcp_port": "5001",
                "serial_port_name": "COM5",
                "serial_baud_rate": "115200",
                "message_on_connect": true
            }
        }
    ]
}
```