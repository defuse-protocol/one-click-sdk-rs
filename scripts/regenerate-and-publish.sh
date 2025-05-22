#!/bin/bash
set -e

# Rust SDK regeneration and publishing script for one-click-sdk-rs
# This script fetches the OpenAPI spec, regenerates the SDK, and publishes it to crates.io

# Define variables
VERSION_NUMBER=${API_VERSION#v}  # Remove leading 'v' if present

SPEC_URL="https://1click.chaindefuser.com/docs/v0/openapi.yaml"

echo "Fetching OpenAPI spec from ${SPEC_URL}..."
curl -s -o "openapi.yaml" "https://1click.chaindefuser.com/docs/v0/openapi.yaml" || { echo "Failed to fetch OpenAPI spec"; exit 1; }

# Install OpenAPI Generator CLI
npm install -g @openapitools/openapi-generator-cli

# Generate the SDK
echo "Generating Go SDK..."
openapi-generator-cli generate \
  -g rust \
  -i "openapi.yaml" \
  -o . \
  --git-user-id defuse-protocol \
  --git-repo-id one-click-sdk-rs \
  --package-name oneclick \
  --output ./v0  || { echo "Failed to generate SDK"; exit 1; }

# Update version in Cargo.toml
echo "Updating Cargo.toml version to ${VERSION_NUMBER}"
sed -i "s/^version = \".*\"/version = \"${VERSION_NUMBER}\"/g" Cargo.toml

# Build and test the SDK
echo "Building Rust SDK..."
cargo build || { echo "Cargo build failed"; exit 1; }

# Commit and push changes
echo "Committing changes..."
git config user.email "action@github.com"
git config user.name "GitHub Action"
git add .
git commit -m "Release: ${VERSION_NUMBER} version" || echo "No changes to commit"

# Verify the package is ready to be published
echo "Verifying package..."
cargo package || { echo "Package verification failed"; exit 1; }

# Publish to crates.io
echo "Publishing to crates.io..."
cargo login "$CARGO_REGISTRY_TOKEN" || { echo "Failed to login to crates.io"; exit 1; }
cargo publish || { echo "Failed to publish package"; exit 1; }

echo "Pushing changes..."
git tag -a "v$VERSION_NUMBER" -m "Release v$VERSION_NUMBER"
git push

echo "Rust SDK successfully regenerated and published for API version ${VERSION_NUMBER}"
