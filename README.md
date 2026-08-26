# Hide Windows on Active Monitor

Overrides **Win+D** to hide/show windows only on the current monitor.

Hold **Win** and tap **D** repeatedly to toggle.

- Per-monitor independent toggle state
- Other Win+X shortcuts unaffected
- System tray with "Start with Windows" (on by default) and "Exit"
- Single instance, no config needed

Completely vibecoded.

## No longer supported 

I migrated to Linux and not interested in supporting the software for OS I no longer use. This utility works decent but still has bugs

## Building

Requires Rust with `x86_64-pc-windows-gnu` target and MinGW:

```sh
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

Binary at `target/x86_64-pc-windows-gnu/release/hide-windows-on-active-monitor.exe`.

## Debugging

Use [DebugView](https://learn.microsoft.com/en-us/sysinternals/downloads/debugview) with **Capture > Capture Win32** enabled.

## License

MIT
