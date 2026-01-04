#!/bin/bash
set -e

echo "🦀 Building Rust WASM..."
cargo build --target wasm32-unknown-unknown --release

echo "📦 Copying WASM to frontend..."
mkdir -p frontend/static
cp target/wasm32-unknown-unknown/release/WebFlappyBird.wasm frontend/static/WebFlappyBird.wasm

echo "🎨 Copying assets to frontend..."
rm -rf frontend/static/assets
cp -r assets frontend/static/

echo "📥 Setting up mq_js_bundle.js..."
cd frontend
if [ ! -f static/mq_js_bundle.js ]; then
    curl -sS https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js -o static/mq_js_bundle.js
fi

echo "🎨 Building frontend..."
npm install
npm run build

echo "✅ Build complete! Frontend build is in frontend/build/"
