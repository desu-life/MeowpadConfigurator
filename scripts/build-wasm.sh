#!/bin/bash
# Build WASM module and copy to public directory

set -e

echo "Building WASM module..."

# Navigate to WASM directory
cd "$(dirname "$0")/../src-wasm"

# Build WASM with wasm-pack
echo "Running wasm-pack build..."
wasm-pack build --target web --out-dir ../wasm --release

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✓ WASM build successful!"
    echo "Output: wasm/"
    
    # List generated files
    echo ""
    echo "Generated files:"
    ls -lh ../wasm/
else
    echo "✗ WASM build failed!"
    exit 1
fi

echo ""
echo "WASM module is ready for use in the web application."
echo "Import it with: import init from '../../wasm/meowpad_wasm.js'"
