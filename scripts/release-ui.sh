#!/bin/bash
# ZERO GUI: Production bundle build
cd crates/ui
pnpm install --frozen-lockfile
pnpm tauri build
echo "GUI build complete."
