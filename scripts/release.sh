#!/bin/bash
# Script to automate the release process
# Usage: ./scripts/release.sh [version]
# Example: ./scripts/release.sh 0.1.53
#
# If no version is provided, uses the version from Cargo.toml

set -e

git pull --ff-only

# Get version from argument or Cargo.toml
if [ -n "$1" ]; then
    VERSION="$1"
else
    VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
fi

echo "📦 Preparing release v${VERSION}..."

# Update version in Cargo.toml if passed as argument
if [ -n "$1" ]; then
    echo "📝 Updating Cargo.toml..."
    sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
fi

# Update version in flake.nix
echo "📝 Updating flake.nix..."
sed -i "s/version = \".*\";.*# update when releasing/version = \"${VERSION}\";  # update when releasing/" flake.nix

# Update Cargo.lock
echo "🔄 Updating Cargo.lock..."
cargo update --workspace

echo "🧪 Running tests before publishing..."
cargo test --locked

# Check for changes
if git diff --quiet; then
    echo "ℹ️ No changes detected"
else
    echo "📤 Committing changes..."
    git add -A
    git commit -m "chore: bump version to v${VERSION}"
fi

# Keep the release commit on the branch as well as the tag.
echo "📤 Pushing release commit..."
git push origin HEAD

# Create tag
echo "🏷️ Creating tag v${VERSION}..."
git tag "v${VERSION}"

# Push the release tag
echo "🚀 Pushing to repository..."
git push origin v${VERSION}

echo "✅ Release v${VERSION} completed successfully!"
echo ""
echo "GitHub workflow will:"
echo "  - Create the GitHub release"
echo "  - Publish to crates.io"
echo "  - Update the APT repository"
echo "  - Build Linux and Windows binaries"
