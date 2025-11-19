#!/bin/bash
set -e

echo "Building WASM package..."
wasm-pack build --target web

echo "Creating distribution folder..."
rm -rf dist
mkdir dist
cp index.html dist/
cp style.css dist/
cp icon.png dist/

# Remove .gitignore from pkg to ensure it's tracked
rm -f pkg/.gitignore

cp -r pkg dist/

echo "Build complete! Artifacts are in the 'dist' folder."
echo "You can serve the project with: python3 -m http.server --directory dist"
