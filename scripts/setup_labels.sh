#!/bin/bash

# ZERO Repository Label Setup
# Aligned with technical governance standards.
# Requires GitHub CLI (gh) installed and authenticated.

if ! command -v gh &> /dev/null; then
    echo "Error: GitHub CLI (gh) is not installed."
    exit 1
fi

if ! gh auth status &> /dev/null; then
    echo "Error: GitHub CLI is not authenticated. Please run 'gh auth login' first."
    exit 1
fi

labels=(
  "security:#D93F0B:Vulnerabilities or systemic security risks"
  "compliance:#0075ca:Regulatory and protocol alignment"
  "optimization:#e99695:Efficiency and performance improvements"
  "critical:#b60205:Urgent systemic failures"
  "zero-friction:#0e8a16:UX smoothness and integration ease"
  "anomaly:#ee0701:Technical deviations or logic failures"
  "integrity:#fbca04:Data correctness and consistency"
  "enhancement:#a2eeef:Functional expansion of the protocol"
)

echo "Syncing ZERO Label Infrastructure..."

for label in "${labels[@]}"; do
  IFS=":" read -r name color description <<< "$label"
  echo "Processing label: $name"
  gh label create "$name" --color "$color" --description "$description" --force
done

echo "Label synchronization complete."