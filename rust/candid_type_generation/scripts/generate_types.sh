#!/bin/bash

set -e

echo "========================================="
echo "Generating Rust types from Candid interface"
echo "========================================="

# Input Candid file
INPUT_FILE="nns_governance.did"

# Output Rust file for generated types
OUTPUT_FILE="src/nns_governance_types.rs"

# Check if input file exists
if [ ! -f "$INPUT_FILE" ]; then
    echo "❌ Error: Candid file '$INPUT_FILE' not found!"
    echo "💡 Run './scripts/fetch_candid.sh' first to fetch the Candid interface"
    exit 1
fi

echo "🔄 Input file: $INPUT_FILE"
echo "📝 Output file: $OUTPUT_FILE"

# Check if didc is installed, if not, try to install it
if ! command -v didc &> /dev/null; then
    echo "🔧 didc not found, attempting to install..."
    if command -v cargo &> /dev/null; then
        cargo install didc
    else
        echo "❌ Error: cargo not found. Please install Rust and cargo first."
        exit 1
    fi
fi

echo "⚙️  Generating Rust types using didc..."

# Generate Rust bindings from the Candid file
# The --target rs flag generates Rust code
didc bind "$INPUT_FILE" --target rs > "$OUTPUT_FILE"

# Check if the file was created successfully
if [ -f "$OUTPUT_FILE" ]; then
    echo "✅ Successfully generated Rust types!"
    echo "📄 Types saved to: $OUTPUT_FILE"
    echo "📏 File size: $(wc -c < "$OUTPUT_FILE") bytes"
    echo "📊 Line count: $(wc -l < "$OUTPUT_FILE") lines"
    
    # Show a preview of the generated types
    echo ""
    echo "📋 Preview of generated types:"
    echo "----------------------------------------"
    head -20 "$OUTPUT_FILE"
    echo "----------------------------------------"
    echo "... (truncated, see full file at $OUTPUT_FILE)"
else
    echo "❌ Failed to generate Rust types"
    exit 1
fi

echo ""

# Step 4: Post-process the generated types
./scripts/postprocess_types.sh

echo "========================================="
echo "✅ Type generation complete!"
echo "========================================="
