#!/bin/bash

# Move to the project root (one level up from "scripts/")
cd "$(dirname "$0")/.." || exit 1

# Check if a module name was provided
if [ -z "$1" ]; then
    echo "❌ Error: Please provide a module name."
    echo "Usage: ./scripts/add_module.sh <module_name>"
    exit 1
fi

MODULE_NAME=$1
MODULE_PATH="./$MODULE_NAME"  # ✅ Create module in the project root
COMMON_PATH="./common"

# Step 1: Create a new Rust binary crate in the correct location
echo "🚀 Creating new Rust module: $MODULE_NAME"
cargo new "$MODULE_PATH"

# Step 2: Add the new module to Cargo.toml workspace members
echo "🔧 Adding $MODULE_NAME to the workspace members..."
sed -i "/members = \[/ a \    \"$MODULE_NAME\"," Cargo.toml

# Step 3: Add `common` as a dependency in the new module
echo "🔗 Adding common as a dependency..."
echo -e "\n[dependencies]\ncommon = { path = \"$COMMON_PATH\" }" >> "$MODULE_PATH/Cargo.toml"

# Step 4: Confirm success
echo "✅ Module $MODULE_NAME added successfully in the project root!"
