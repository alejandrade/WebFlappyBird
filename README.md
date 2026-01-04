# MiniFlappyBird 🐦

A clean Flappy Bird clone built in **Rust** with WebAssembly support. Play it natively on desktop or in your browser!

**[🎮 Play Online](https://alejandrade.github.io/WebFlappyBird/)** | **[☕ Support the Project](https://ko-fi.com/alejandrade)**

## 🎯 How to Play

Navigate your bird through an endless series of pipes by tapping or clicking to flap. Each successful pass through a pipe earns you a point. The game gets progressively harder as you score more points!

### Controls
* **Desktop:** Press `SPACE` or `LEFT CLICK` to flap
* **Mobile:** Tap the screen to flap
* **Esc:** Quit the game (desktop only)

### Tips
* Timing is everything - don't spam the flap button!
* The bird gains momentum as it falls, so plan your flaps accordingly
* Try to stay in the middle of the screen when possible
* Your high score is saved automatically

## 🚀 Getting Started

### Play in Browser
Simply visit **[alejandrade.github.io/WebFlappyBird](https://alejandrade.github.io/WebFlappyBird/)** to play instantly - no installation required!

### Run Locally (Desktop)

#### Prerequisites
* Rust toolchain - Get it from [rustup.rs](https://rustup.rs)

#### How to Run
```bash
cargo run
```

That's it! Cargo will handle all dependencies automatically.

### Build for Web (WASM)

#### Prerequisites
* Rust toolchain
* Node.js and npm
* Basic tools: `cargo install basic-http-server`

#### Build Steps
```bash
# Build the Rust WASM module
./build.sh

# Navigate to the frontend and run dev server
cd frontend
npm install
npm run dev
```

The game will be available at `http://localhost:5173`

### Deploy to GitHub Pages

This repository includes a GitHub Actions workflow that automatically builds and deploys the game to GitHub Pages when you push to the `master` branch.

#### Enable GitHub Pages:
1. Go to your repository Settings
2. Navigate to **Pages** in the left sidebar
3. Under **Source**, select **GitHub Actions**
4. Push to the `master` branch and the workflow will automatically deploy

Your game will be available at: `https://<username>.github.io/WebFlappyBird/`

## 🛠️ Built With

* **[Macroquad](https://github.com/not-fl3/macroquad)** - Simple and easy-to-use game library for Rust
* **[quad-snd](https://github.com/not-fl3/quad-snd)** - Audio playback for Macroquad
* **WebAssembly** - For browser compatibility
* **SvelteKit** - Frontend framework for web deployment

## 🎨 Features

* ✨ Pixel-perfect retro graphics
* 🎵 Background music with fade transitions
* 🔊 Sound effects for flapping, scoring, and collisions
* 📱 Fully responsive - works on desktop and mobile
* 💾 Automatic high score tracking
* 🌐 Runs natively or in browser via WebAssembly

## 📁 Project Structure

```
WebFlappyBird/
├── src/                    # Rust game source code
│   ├── main.rs            # Entry point and game loop
│   ├── game_state.rs      # Game state management
│   ├── player.rs          # Bird/player logic
│   ├── music_player.rs    # Background music system
│   └── ...                # Other game modules
├── frontend/              # Web frontend (SvelteKit)
│   ├── static/            # Static assets (sprites, audio, WASM)
│   └── src/               # SvelteKit source
└── assets/                # Game assets (used for native build)
```

## ☕ Support

If you enjoy this game, consider [buying me a coffee](https://ko-fi.com/alejandrade)! Your support helps me create more open-source projects.

## 📜 License

This project is licensed under the **CC BY-NC-SA 4.0** (Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International).

You are free to:
* Share and adapt the code
* Use it for learning and personal projects

Under the following terms:
* Give appropriate credit
* Non-commercial use only
* Share adaptations under the same license

See the [LICENSE](LICENSE) file for details.

---

Made with ❤️ and Rust 🦀