# Hide Windows on Active Monitor

A lightweight Windows background utility that overrides **Win+D** to hide/show windows **only on the current monitor**, instead of minimizing all windows across all monitors.

## Features

- **Per-monitor Win+D**: Each monitor has independent show/hide state
- **Hold Win + tap D repeatedly** to quickly toggle hide/restore on the active monitor
- **Spanning windows**: Windows overlapping the target monitor are included
- **Original z-order preserved**: Windows are restored in the correct stacking order
- **Other Win+X shortcuts unaffected**: Win+E, Win+Tab, Win+R, etc. all work normally
- **System tray**: Right-click for "Start with Windows" (enabled by default) and "Exit"
- **Single instance**: Named mutex prevents multiple copies from running
- **No config needed**: Just run it

## How It Works

- Installs a `WH_KEYBOARD_LL` low-level keyboard hook to intercept Win+D
- Detects the active monitor via foreground window (falls back to cursor position when desktop is active)
- Uses `EnumWindows` to collect visible app windows overlapping the target monitor
- Minimizes with `SW_SHOWMINNOACTIVE` to prevent activating windows on other monitors
- Restores in reverse z-order to preserve the original stacking order

## Building

Requires Rust with the `x86_64-pc-windows-gnu` target and MinGW:

```sh
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The binary will be at `target/x86_64-pc-windows-gnu/release/hide-windows-on-active-monitor.exe`.

### Debugging

The app outputs debug logs via `OutputDebugStringW`. Use [DebugView](https://learn.microsoft.com/en-us/sysinternals/downloads/debugview) (Sysinternals) with **Capture > Capture Win32** enabled to view them.

## Usage

1. Run the exe (it starts silently in the system tray)
2. Press **Win+D** to hide windows on the current monitor
3. Press **Win+D** again to restore them
4. Hold **Win** and tap **D** repeatedly for quick toggle
5. Right-click the tray icon for options

## Auto-Start

Enabled by default. Registers in `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`. Toggle via the tray menu.

## Architecture

```
src/
  main.rs    - Entry point, hidden window, WndProc, monitor detection, window filtering
  hook.rs    - WH_KEYBOARD_LL hook, Win+D detection and suppression
  window.rs  - EnumWindows with monitor rect intersection
  state.rs   - Per-monitor toggle state, minimize/restore logic
  tray.rs    - System tray icon and context menu
  autostart.rs - Registry auto-start management
  debug.rs   - OutputDebugStringW logging
```

## License

MIT
