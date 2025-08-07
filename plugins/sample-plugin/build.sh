#!/bin/bash

# Sample Go build script for Build Studio plugin

# Check if Go is installed
go version > /dev/null 2>&1 || {
  echo "Error: Go is not installed or not in PATH"
  exit 1
}

# Get parameters from Build Studio
PLATFORM=$1
OUTPUT_DIR=$2

# Set GOOS based on platform
GOOS=""
case $PLATFORM in
  "linux")
    GOOS="linux"
    ;;
  "windows")
    GOOS="windows"
    ;;
  "macos")
    GOOS="darwin"
    ;;
  "webassembly")
    GOOS="js"
    GOARCH="wasm"
    ;;
  *)
    echo "Unsupported platform: $PLATFORM"
    exit 1
    ;;
esac

# Build the Go project
if [ "$GOOS" = "js" ] && [ "$GOARCH" = "wasm" ]; then
  echo "Building WebAssembly module..."
  GOOS=$GOOS GOARCH=$GOARCH go build -o $OUTPUT_DIR/main.wasm .
else
  echo "Building for $GOOS..."
  GOOS=$GOOS go build -o $OUTPUT_DIR/main .
fi

echo "Build completed successfully for $PLATFORM"
