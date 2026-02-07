#!/bin/bash

echo "🔧 Fixing workflow triggers..."

# Function to add workflow_dispatch to a workflow file
add_workflow_dispatch() {
    local file=$1
    local name=$2
    
    echo "📝 Fixing $file..."
    
    # Check if workflow_dispatch already exists
    if grep -q "workflow_dispatch:" "$file"; then
        echo "  ✓ Already has workflow_dispatch"
        return
    fi
    
    # Extract the 'on:' section and add workflow_dispatch
    # This is a bit tricky with YAML, so we'll do it carefully
    
    # Create a temp file
    temp_file=$(mktemp)
    
    # Read the file and insert workflow_dispatch after 'on:'
    awk '
    BEGIN { found_on = 0; added_dispatch = 0 }
    /^on:/ { 
        print $0
        found_on = 1
        next
    }
    found_on && !added_dispatch && /^[a-zA-Z]/ {
        print "  workflow_dispatch:"
        added_dispatch = 1
    }
    { print }
    ' "$file" > "$temp_file"
    
    # Move temp file back
    mv "$temp_file" "$file"
    echo "  ✓ Added workflow_dispatch"
}

# Fix all workflow files
cd .github/workflows || exit 1

for file in *.yml; do
    # Skip system files
    if [[ "$file" == "_"* ]]; then
        continue
    fi
    
    add_workflow_dispatch "$file" "${file%.yml}"
done

cd ../..

echo ""
echo "✅ Workflow triggers fixed!"
echo ""
echo "📋 Verifiable workflows:"
grep -l "workflow_dispatch:" .github/workflows/*.yml | xargs -I {} basename {}