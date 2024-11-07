<div align="center">
  <img src="app-icon.png" alt="NeoHtop Logo" width="120" />
  <h1>NeoHtop</h1>
  <p>A modern, cross-platform system monitor built on top of Svelte, Rust, and Tauri.</p>

  [![License](https://img.shields.io/github/license/Abdenasser/neohtop)](https://github.com/Abdenasser/neohtop/blob/main/LICENSE)
  [![GitHub stars](https://img.shields.io/github/stars/Abdenasser/neohtop)](https://github.com/Abdenasser/neohtop/stargazers)
  [![GitHub issues](https://img.shields.io/github/issues/Abdenasser/neohtop)](https://github.com/Abdenasser/neohtop/issues)
  [![GitHub release](https://img.shields.io/github/v/release/Abdenasser/neohtop)](https://github.com/Abdenasser/neohtop/releases)
  [![Notarized by Apple](https://img.shields.io/badge/Release_Notarized_by_Apple-000000?style=flat-square&logo=apple&logoColor=white)](https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution)
</div>

<div align="center">
  <img src="https://github.com/Abdenasser/neohtop/blob/main/screenshot.png?raw=true" alt="NeoHtop Screenshot" width="800" />
</div>

## [Why another system monitor? That's not a TUI? (The Back Story)](https://www.abdenasser.com/2024/11/06/oh-boy-neohtop/)

## Features

- 🚀 Real-time process monitoring
- 💻 CPU and Memory usage tracking
- 🎨 Beautiful, modern UI with dark/light themes
- 🔍 Process search and filtering
- 📌 Pin important processes
- 🛠 Process management (kill processes)
- 🎯 Sort by any column
- 🔄 Auto-refresh system stats

## Tech Stack

- **Frontend**: SvelteKit, TypeScript
- **Backend**: Rust, Tauri
- **Styling**: CSS Variables for theming
- **Icons**: FontAwesome

## Development

### Prerequisites

- Node.js (v16 or later)
- Rust (latest stable)
- Xcode Command Line Tools (for macOS)

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Contributing

We welcome contributions from the community! Please see our [contributing guidelines](./.github/CONTRIBUTING.md) for more information.

## Support

If you find this project helpful, consider buying me a coffee:

<a href="https://www.buymeacoffee.com/abdenasser" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;" ></a>

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
