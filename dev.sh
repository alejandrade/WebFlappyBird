#!/bin/bash
set -e

echo "🦀 Building Rust WASM..."
cargo build --target wasm32-unknown-unknown --release

echo "📦 Copying WASM to frontend..."
cp target/wasm32-unknown-unknown/release/WebFlappyBird.wasm frontend/static/

echo "🎨 Copying assets to frontend..."
rm -rf frontend/static/assets
cp -r assets frontend/static/

echo "📥 Setting up gl.js..."
cd frontend
if [ ! -f static/gl.js ]; then
    curl -sS https://raw.githubusercontent.com/not-fl3/miniquad/master/js/gl.js -o static/gl.js
fi

echo "📦 Installing dependencies..."
npm install

echo "🚀 Starting dev server..."
npm run dev