#!/bin/bash

# Check if a version argument is provided
if [ $# -eq 0 ]; then
    echo "Please provide a version number as an argument."
    echo "Usage: $0 <version_number>"
    exit 1
fi

# Store the new version
NEW_VERSION="$1"

# Detect OS and set the appropriate sed command
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    if command -v gsed >/dev/null 2>&1; then
        SED_CMD="gsed"
    else
        echo "Error: gsed is not installed. Please install it using Homebrew: brew install gnu-sed"
        exit 1
    fi
else
    # Linux and other Unix-like systems
    SED_CMD="sed"
fi

# Find all package.json files in subdirectories
find . -name "package.json" | while read -r file; do
    # Check if the file contains any "@dfinity/" package
    if grep -q '"@dfinity/' "$file"; then
        # Use sed to replace the version for all @dfinity packages, preserving the caret if present
        $SED_CMD -i 's/\("@dfinity\/[^"]*"\s*:\s*"\^*\)[0-9.]*"/\1'"$NEW_VERSION"'"/' "$file"
        echo "Updated @dfinity/* versions in $file"
    fi
done

echo "Script completed."
