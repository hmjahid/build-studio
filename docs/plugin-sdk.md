# Build Studio Plugin SDK

## Overview

Build Studio supports a plugin system that allows developers to extend the application with custom build toolchains, packaging methods, and other functionality. Plugins are loaded dynamically from the `plugins/` directory at runtime.

## Plugin Structure

A Build Studio plugin consists of:

1. A plugin manifest file (`plugin.json` or `plugin.yaml`)
2. Implementation files (scripts, binaries, or code)
3. Optional assets and resources

## Plugin Manifest

The plugin manifest describes the plugin to Build Studio. It must be named `plugin.json` or `plugin.yaml` and placed in the plugin's root directory.

### JSON Manifest Format

```json
{
  "name": "Plugin Name",
  "author": "Author Name",
  "version": "1.0.0",
  "description": "Brief description of what the plugin does",
  "commands": [
    {
      "name": "command-name",
      "description": "Description of what the command does",
      "script": "path/to/script.sh"
    }
  ],
  "platforms": ["linux", "windows", "macos", "android", "ios", "webassembly"],
  "dependencies": [
    "required-tool>=1.0.0"
  ]
}
```

### YAML Manifest Format

```yaml
name: Plugin Name
author: Author Name
version: 1.0.0
description: Brief description of what the plugin does
commands:
  - name: command-name
    description: Description of what the command does
    script: path/to/script.sh
platforms:
  - linux
  - windows
  - macos
  - android
  - ios
  - webassembly
dependencies:
  - required-tool>=1.0.0
```

## Plugin Implementation

Plugins can be implemented in any language that can be executed on the target platform. Common approaches include:

1. Shell scripts (bash, batch, etc.)
2. Python scripts
3. Node.js scripts
4. Compiled binaries

### Example Shell Script Plugin

```bash
#!/bin/bash

# Get parameters from Build Studio
COMMAND=$1
PLATFORM=$2
SOURCE_DIR=$3
OUTPUT_DIR=$4

# Execute the command
case $COMMAND in
  "build")
    echo "Building for $PLATFORM"
    # Add build logic here
    ;;
  "clean")
    echo "Cleaning build artifacts"
    # Add clean logic here
    ;;
  *)
    echo "Unknown command: $COMMAND"
    exit 1
    ;;
esac
```

## Plugin API

Plugins communicate with Build Studio through:

1. Command-line arguments
2. Environment variables
3. Standard input/output streams
4. File system operations

### Command-Line Arguments

Plugins receive the following arguments:

1. Command name
2. Target platform
3. Source directory
4. Output directory
5. Additional parameters as needed

### Environment Variables

Build Studio sets the following environment variables:

- `BUILD_STUDIO_VERSION`: Version of Build Studio
- `PLUGIN_DIR`: Path to the plugin directory
- `PROJECT_DIR`: Path to the current project directory

## Packaging and Distribution

Plugins should be distributed as:

1. A zip archive containing all plugin files
2. A git repository that users can clone
3. A package in a plugin registry (future feature)

## Best Practices

1. Always check for dependencies before executing
2. Handle errors gracefully and provide meaningful error messages
3. Use the output directory for all generated files
4. Clean up temporary files after execution
5. Follow platform-specific conventions
6. Document your plugin thoroughly

## Example Plugin

See the `sample-plugin` directory in the Build Studio repository for a complete example.
