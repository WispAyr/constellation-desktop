# Constellation Desktop

Native macOS desktop application for OpenPlatform portal, built with Tauri 2.0.

## Features

- 🖥️ Native macOS app wrapper for OpenPlatform
- 💾 Remembers window size and position
- 🎨 Clean, minimal native feel
- ⚡ Lightweight (~15MB bundle)

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install)
- Xcode Command Line Tools

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Build

```bash
# Build release .app
npm run tauri build

# Built app location: src-tauri/target/release/bundle/macos/Constellation.app
```

## Configuration

The app connects to OpenPlatform at `http://192.168.195.33:3100`.

To change the URL, edit:
- `src/index.html` - Update `OPENPLATFORM_URL` constant
- `src-tauri/src/lib.rs` - Update `OPENPLATFORM_URL` constant
- `src-tauri/capabilities/default.json` - Update `remote.urls`
- `src-tauri/tauri.conf.json` - Update CSP headers

## Project Structure

```
constellation-desktop/
├── src/                    # Frontend (splash/redirect page)
│   └── index.html
├── src-tauri/             # Rust/Tauri backend
│   ├── capabilities/      # Security permissions
│   ├── icons/             # App icons
│   ├── src/
│   │   ├── lib.rs         # Main Tauri setup
│   │   └── main.rs        # Entry point
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
└── package.json           # Node.js config
```

## License

Proprietary - Parkwise Technologies Ltd
