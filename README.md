# Pacman Rust WASM

A Pacman clone written in Rust and compiled to WebAssembly (WASM).

## 🎮 Demo
**Play the game here:** [https://mango-smoke-041c5121e.3.azurestaticapps.net](https://mango-smoke-041c5121e.3.azurestaticapps.net)

## ✨ Features
- Classic Pacman gameplay
- Written in Rust 🦀
- Runs in the browser via WebAssembly ⚡
- **Mobile Support**: Swipe controls for touch devices 📱

## 🛠️ Build & Run Locally

### Prerequisites
- Rust and Cargo
- `wasm-pack`:
  ```bash
  cargo install wasm-pack
  ```

### Run
1.  **Build the project:**
    ```bash
    ./build.sh
    ```
    This will compile the Rust code to WASM and create a `dist` folder.

2.  **Serve the game:**
    ```bash
    python3 -m http.server --directory dist
    ```

3.  Open `http://localhost:8000` in your browser.

## 🚀 Deployment
This project is configured to deploy automatically to Azure Static Web Apps via GitHub Actions.
Any push to the `main` branch triggers a new build and deployment.
