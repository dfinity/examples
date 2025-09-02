#!/bin/bash

set -e

echo "🔧 Post-processing generated Rust types..."

TYPES_FILE="src/nns_governance_types.rs"

if [ ! -f "$TYPES_FILE" ]; then
    echo "❌ Error: Types file '$TYPES_FILE' not found!"
    echo "💡 Run './scripts/generate_types.sh' first"
    exit 1
fi

echo "📝 Processing: $TYPES_FILE"

# Create a backup
cp "$TYPES_FILE" "$TYPES_FILE.backup"

# Fix the import path - remove the deprecated ic_cdk::export:: prefix
sed -i.tmp 's/use ic_cdk::export::candid/use candid/g' "$TYPES_FILE"

# Make all structs and enums public by adding 'pub' before 'struct' and 'enum'
sed -i.tmp 's/^struct /pub struct /g' "$TYPES_FILE"
sed -i.tmp 's/^enum /pub enum /g' "$TYPES_FILE"

# Add Debug and Serialize derives to all types by replacing the CandidType, Deserialize derive
sed -i.tmp 's/#\[derive(CandidType, Deserialize)\]/#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]/g' "$TYPES_FILE"

# Make struct fields public by adding 'pub' to field declarations  
# Target lines that start with whitespace + identifier + colon (struct fields)
# but exclude function parameters by being more specific
sed -i.tmp 's/^\(  \)\([a-z_][a-zA-Z0-9_]*\): /\1pub \2: /' "$TYPES_FILE"

# Remove the temporary file created by sed
rm -f "$TYPES_FILE.tmp"

echo "✅ Post-processing complete!"
echo "🔧 Applied fixes:"
echo "  • Updated import paths to use modern candid crate"
echo "  • Made all types public (added 'pub' keyword)"
echo "  • Made all struct fields public (added 'pub' keyword)"
echo "  • Added Debug, Clone, and Serialize derives to all types"
echo "📁 Backup saved to: $TYPES_FILE.backup"
