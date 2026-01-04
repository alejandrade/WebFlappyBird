#!/bin/bash
set -e

echo "🦀 Building Rust WASM..."
cargo build --target wasm32-unknown-unknown --release

echo "📦 Copying WASM to frontend..."
cp target/wasm32-unknown-unknown/release/WebFlappyBird.wasm frontend/static/

echo "🎨 Copying assets to frontend..."
rm -rf frontend/static/assets
cp -r assets frontend/static/

echo "📥 Setting up mq_js_bundle.js..."
cd frontend
curl -sS https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js -o static/mq_js_bundle.js

echo "📦 Installing dependencies..."
npm install

echo "🚀 Starting dev server..."
npm run dev