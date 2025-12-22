#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version provided
if [ -z "$1" ]; then
  echo -e "${RED}Error: Version required${NC}"
  echo "Usage: ./scripts/prepare-release.sh 0.2.0"
  exit 1
fi

VERSION=$1
TAG="v${VERSION}"

echo -e "${GREEN}Preparing release ${TAG}${NC}\n"

# Verify we're on main branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$CURRENT_BRANCH" != "main" ]; then
  echo -e "${YELLOW}Warning: Not on main branch (currently on ${CURRENT_BRANCH})${NC}"
  read -p "Continue anyway? (y/N) " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
  fi
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
  echo -e "${RED}Error: Uncommitted changes detected${NC}"
  echo "Commit or stash changes before releasing"
  exit 1
fi

# Update version in Cargo.toml
echo -e "${GREEN}[1/6] Updating Cargo.toml version...${NC}"
sed -i.bak "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
rm Cargo.toml.bak

# Update version in engine workspace Cargo.toml
echo -e "${GREEN}[2/6] Updating engine/Cargo.toml version...${NC}"
sed -i.bak "s/^version = \".*\"/version = \"${VERSION}\"/" engine/Cargo.toml
rm engine/Cargo.toml.bak

# Update version in Tauri config
echo -e "${GREEN}[3/6] Updating Tauri config version...${NC}"
sed -i.bak "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" apps/desktop/src-tauri/tauri.conf.json
rm apps/desktop/src-tauri/tauri.conf.json.bak

# Check if release notes exist
RELEASE_NOTES_FILE=".github/release-notes/${TAG}.md"
if [ ! -f "$RELEASE_NOTES_FILE" ]; then
  echo -e "${YELLOW}[4/6] Release notes not found, creating from template...${NC}"
  cp .github/release-notes/TEMPLATE.md "$RELEASE_NOTES_FILE"

  # Replace version placeholder
  sed -i.bak "s/v{VERSION}/${TAG}/g" "$RELEASE_NOTES_FILE"
  rm "${RELEASE_NOTES_FILE}.bak"

  echo -e "${YELLOW}Please edit release notes: ${RELEASE_NOTES_FILE}${NC}"
  read -p "Press Enter to open in editor..." -r
  ${EDITOR:-nano} "$RELEASE_NOTES_FILE"
else
  echo -e "${GREEN}[4/6] Release notes found: ${RELEASE_NOTES_FILE}${NC}"
fi

# Commit version bump
echo -e "${GREEN}[5/6] Committing version bump...${NC}"
git add Cargo.toml engine/Cargo.toml apps/desktop/src-tauri/tauri.conf.json "$RELEASE_NOTES_FILE"
git commit -m "chore: bump version to ${VERSION}"

# Create tag
echo -e "${GREEN}[6/6] Creating tag ${TAG}...${NC}"
git tag -a "$TAG" -m "Release ${TAG}"

echo -e "\n${GREEN}✓ Release prepared successfully!${NC}\n"
echo "Next steps:"
echo "  1. Review changes: git show HEAD"
echo "  2. Push to trigger release: git push origin main && git push origin ${TAG}"
echo "  3. Monitor workflow: https://github.com/HelixoidLLC/pmsynapse-mono/actions"
echo "  4. Review draft release: https://github.com/HelixoidLLC/pmsynapse/releases"
echo "  5. Publish when ready"
echo ""
echo -e "${YELLOW}Note: Release will be created as DRAFT in public repo${NC}"
