#!/usr/bin/env bash
set -euo pipefail

LEVEL="${1:?Usage: release.sh <patch|minor|major> [--dry-run]}"
DRY_RUN="${2:-}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Validate level
if [[ "$LEVEL" != "patch" && "$LEVEL" != "minor" && "$LEVEL" != "major" ]]; then
  echo "Error: level must be patch, minor, or major"
  exit 1
fi

# Ensure working directory is clean
if [[ -n "$(git -C "$ROOT" status --porcelain)" ]]; then
  echo "Error: working directory is not clean. Commit or stash changes first."
  exit 1
fi

# Ensure on main branch
BRANCH="$(git -C "$ROOT" branch --show-current)"
if [[ "$BRANCH" != "main" ]]; then
  echo "Error: releases must be made from the main branch (current: $BRANCH)"
  exit 1
fi

# Get current version from Cargo.toml workspace
CURRENT_VERSION="$(sed -n '/\[workspace\.package\]/,/\[/{s/^version = "\(.*\)"/\1/p}' "$ROOT/Cargo.toml")"
if [[ -z "$CURRENT_VERSION" ]]; then
  echo "Error: could not read version from Cargo.toml"
  exit 1
fi
echo "Current version: $CURRENT_VERSION"

# Calculate next version
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"
case "$LEVEL" in
  patch) PATCH=$((PATCH + 1)) ;;
  minor) MINOR=$((MINOR + 1)); PATCH=0 ;;
  major) MAJOR=$((MAJOR + 1)); MINOR=0; PATCH=0 ;;
esac
NEXT_VERSION="${MAJOR}.${MINOR}.${PATCH}"
echo "Next version: $NEXT_VERSION"

if [[ "$DRY_RUN" == "--dry-run" ]]; then
  echo ""
  echo "=== DRY RUN ==="
  echo "Would bump: $CURRENT_VERSION → $NEXT_VERSION"
  echo ""
  echo "=== Files to update ==="
  echo "  $ROOT/Cargo.toml (workspace version)"
  for pkg in "$ROOT"/packages/*/package.json; do
    if echo "$pkg" | grep -q "benchmark"; then continue; fi
    echo "  $pkg"
  done
  echo ""
  echo "=== CHANGELOG preview ==="
  git-cliff --unreleased --tag "v${NEXT_VERSION}" 2>/dev/null || echo "(no conventional commits found)"
  exit 0
fi

# Bump Cargo.toml workspace version
sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEXT_VERSION\"/" "$ROOT/Cargo.toml"
echo "Updated Cargo.toml workspace version"

# Bump all package.json files (skip benchmark)
for pkg in "$ROOT"/packages/*/package.json; do
  if echo "$pkg" | grep -q "benchmark"; then continue; fi
  sed -i "s/\"version\": \"$CURRENT_VERSION\"/\"version\": \"$NEXT_VERSION\"/" "$pkg"
  echo "Updated $pkg"
done

# Update Cargo.lock
cargo check --quiet --manifest-path "$ROOT/Cargo.toml" 2>/dev/null || true

# Generate CHANGELOG
git-cliff --tag "v${NEXT_VERSION}" -o "$ROOT/CHANGELOG.md"
echo "Generated CHANGELOG.md"

# Commit, tag, push
git -C "$ROOT" add -A
git -C "$ROOT" commit -m "release: v${NEXT_VERSION}"
git -C "$ROOT" tag "v${NEXT_VERSION}"
git -C "$ROOT" push --follow-tags

echo ""
echo "Released v${NEXT_VERSION}"
echo "GitHub Actions will now publish to crates.io and npm."
