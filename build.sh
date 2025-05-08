#!/bin/bash
set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo not found. Please install Rust from https://rustup.rs/${NC}"
    exit 1
fi

# Determine the appropriate WebAssembly target
echo -e "${GREEN}Checking supported WebAssembly targets...${NC}"

AVAILABLE_TARGETS=$(rustc --print=target-list)
WASM_TARGET=""

if echo "$AVAILABLE_TARGETS" | grep -q "wasm32-wasip1"; then
    WASM_TARGET="wasm32-wasip1"
    echo -e "${GREEN}Using wasm32-wasip1 target${NC}"
elif echo "$AVAILABLE_TARGETS" | grep -q "wasm32-wasi"; then
    WASM_TARGET="wasm32-wasi"
    echo -e "${GREEN}Using wasm32-wasi target${NC}"
else
    echo -e "${RED}Error: No supported WebAssembly target found. Your Rust toolchain must support either wasm32-wasi or wasm32-wasip1.${NC}"
    exit 1
fi

# Check if the target is installed
if ! rustup target list | grep -q "$WASM_TARGET (installed)"; then
    echo -e "${YELLOW}The $WASM_TARGET target is not installed. Installing now...${NC}"
    rustup target add "$WASM_TARGET"
fi

echo -e "${GREEN}Building the Buildkite MCP extension for Zed using $WASM_TARGET...${NC}"

# Build the extension
cargo build --target "$WASM_TARGET" --release

# Check if the build was successful
if [ $? -eq 0 ]; then
    WASM_PATH="target/$WASM_TARGET/release/mcp_server_buildkite.wasm"
    if [ -f "$WASM_PATH" ]; then
        echo -e "${GREEN}Build successful!${NC}"
        echo -e "${GREEN}WASM file: ${YELLOW}${WASM_PATH}${NC}"
        echo -e "${GREEN}To install in Zed, go to: File > Extensions > Install From Path${NC}"

        # Create the dist directory if it doesn't exist
        mkdir -p dist

        # Copy the extension files to the dist directory
        cp "$WASM_PATH" dist/
        cp extension.toml dist/
        cp README.md dist/

        echo -e "${GREEN}Installation files copied to dist/ directory${NC}"
    else
        echo -e "${RED}Build was successful but WASM file not found at: ${WASM_PATH}${NC}"
        exit 1
    fi
else
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi
