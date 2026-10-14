# Project Color Picker on macOS

Build the app on a Mac (needs Rust from https://rustup.rs):

```
bash macos/build-app.sh
```

That makes `dist/Project Color Picker.app`, which runs on Apple Silicon and Intel Macs (macOS 10.15+).
It isn't signed with a developer account, so the first time you open it, right-click it and choose **Open**.
`cargo run` works too.

Differences from Windows:
- macOS draws the title bar and window buttons; the dim and pin toggles sit just below it
- **Pick Screen** (or **P**) opens macOS's own color sampler: click anywhere to pick, **Esc** to cancel
- **Cmd** works wherever **Ctrl** does (Cmd+C / Cmd+V)
- saved colors and settings live in `~/Library/Application Support/Project Color Picker`
