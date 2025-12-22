# PMSynapse Release Process

This document describes how to create and publish releases of the PMSynapse CLI tool.

## Overview

**Source Repository (Private):** `git@github.com:HelixoidLLC/pmsynapse-mono`
**Public Repository (Artifacts Only):** `https://github.com/HelixoidLLC/pmsynapse`

Releases are built in the private repository and published to the public repository using GitHub Actions. All releases are created as **drafts** for manual review before publishing.

## Prerequisites

### One-Time Setup

#### 1. Create Personal Access Token (PAT)

1. Go to GitHub Settings → Developer settings → Personal access tokens → Fine-grained tokens
2. Click "Generate new token"
3. Configure:
   - **Name:** "PMSynapse Release Publisher"
   - **Expiration:** 1 year (rotate annually)
   - **Repository access:** Only select repositories
     - Select: `HelixoidLLC/pmsynapse`
   - **Permissions:**
     - Contents: Read and write
4. Generate and copy the token

#### 2. Add Secret to Private Repository

1. Go to private repo: `HelixoidLLC/pmsynapse-mono`
2. Settings → Secrets and variables → Actions
3. New repository secret:
   - **Name:** `PUBLIC_REPO_TOKEN`
   - **Value:** [paste your PAT]

#### 3. Verify Public Repository Exists

Ensure `https://github.com/HelixoidLLC/pmsynapse` exists and contains:
- `README.md` (installation instructions)
- `install.sh` (Linux/macOS installer)
- `install.ps1` (Windows installer)
- `LICENSE`

## Pre-Release Checklist

**IMPORTANT:** Run these checks before creating a release tag to avoid CI failures:

```bash
# Run all pre-push checks (recommended)
make check-test
```

This runs:
- `cargo fmt --all -- --check` - Verify code formatting
- `cargo clippy -p snps-core -p snps-cli --all-targets -- -D warnings` - Lint checks
- `cargo test -p snps-core -p snps-cli --all-features` - Run tests

**Manual alternative:**

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-features --workspace --exclude pmsynapse-desktop

# Build to verify
cargo build --release -p snps-cli
```

**If any check fails:**
- Fix the issues before proceeding
- Commit the fixes
- Re-run `make check-test` until all checks pass

## Release Workflow

### Step 1: Prepare Release

Use the automated script:

```bash
./scripts/prepare-release.sh 0.2.0
```

This script:
1. ✓ Verifies you're on main branch
2. ✓ Checks for uncommitted changes
3. ✓ Updates version in `Cargo.toml`
4. ✓ Updates version in `engine/snps-cli/Cargo.toml`
5. ✓ Updates version in `apps/desktop/src-tauri/tauri.conf.json`
6. ✓ Creates/edits release notes (`.github/release-notes/v0.2.0.md`)
7. ✓ Commits version bump
8. ✓ Creates git tag `v0.2.0`

**Manual alternative:**

```bash
# 1. Update versions manually
vim Cargo.toml
vim engine/snps-cli/Cargo.toml
vim apps/desktop/src-tauri/tauri.conf.json

# 2. Create release notes
cp .github/release-notes/TEMPLATE.md .github/release-notes/v0.2.0.md
vim .github/release-notes/v0.2.0.md

# 3. Commit and tag
git add .
git commit -m "chore: bump version to 0.2.0"
git tag v0.2.0
```

### Step 2: Trigger Release Build

```bash
# Push commit and tag to trigger workflow
git push origin main
git push origin v0.2.0
```

**What happens:**
1. GitHub Actions detects tag push
2. Workflow `.github/workflows/release-cli.yml` starts
3. Builds CLI for 5 platforms in parallel:
   - `snps-linux-amd64.tar.gz`
   - `snps-linux-arm64.tar.gz`
   - `snps-macos-amd64.tar.gz`
   - `snps-macos-arm64.tar.gz`
   - `snps-windows-amd64.exe.zip`
4. Creates SHA256 checksums for each
5. Publishes draft release to **public repo**

### Step 3: Monitor Build

```bash
# View workflow progress
open "https://github.com/HelixoidLLC/pmsynapse-mono/actions"
```

**Duration:** ~5-10 minutes

**On success:**
- Workflow shows green checkmark
- Summary shows published artifacts
- Email notification sent

**On failure:**
- Check workflow logs for errors
- Fix issue in source
- Delete tag and retry:
  ```bash
  git tag -d v0.2.0
  git push origin :refs/tags/v0.2.0
  # Fix issue, then re-run prepare-release.sh
  ```

### Step 4: Review Draft Release

```bash
# Open public repo releases page
open "https://github.com/HelixoidLLC/pmsynapse/releases"
```

Verify:
- ✓ Version number correct (v0.2.0)
- ✓ Release notes accurate
- ✓ All 10 files attached (5 archives + 5 checksums)
- ✓ File sizes reasonable (~3-5 MB per archive)
- ✓ Download links work

### Step 5: Publish Release

1. Click **Edit** on the draft release
2. Review one final time
3. Uncheck "Set as a pre-release" (unless it's a beta)
4. Click **Publish release**

**Now live:** Users can download from:
- `https://github.com/HelixoidLLC/pmsynapse/releases/latest`
- Direct download URLs work

## Manual Trigger (Advanced)

Release without creating a tag first:

1. Go to: `https://github.com/HelixoidLLC/pmsynapse-mono/actions/workflows/release-cli.yml`
2. Click "Run workflow"
3. Enter tag (e.g., `v0.2.0`)
4. Click "Run workflow"

**Use case:** Re-release same version with fixes

## Artifacts Produced

| Platform | Archive | Checksum | Binary Inside |
|----------|---------|----------|---------------|
| Linux x64 | `snps-linux-amd64.tar.gz` | `.sha256` | `snps` |
| Linux ARM64 | `snps-linux-arm64.tar.gz` | `.sha256` | `snps` |
| macOS x64 | `snps-macos-amd64.tar.gz` | `.sha256` | `snps` |
| macOS ARM64 | `snps-macos-arm64.tar.gz` | `.sha256` | `snps` |
| Windows x64 | `snps-windows-amd64.exe.zip` | `.sha256` | `snps.exe` |

**Binary details:**
- Statically linked (no external dependencies)
- Debug symbols stripped
- Optimized with `--release`
- Size: ~3-5 MB per platform

## Release Notes Best Practices

**Structure** (`.github/release-notes/v{VERSION}.md`):

```markdown
## PMSynapse CLI v{VERSION}

Brief description of the release.

### Features
- New feature 1
- New feature 2

### Bug Fixes
- Fixed issue 1
- Fixed issue 2

### Breaking Changes
- Breaking change 1 (if any)

### Installation
[Standard installation instructions]

### Documentation
Links to relevant docs
```

**Tips:**
- Focus on **user-facing changes** (not internal refactoring)
- Use **present tense** ("Adds X" not "Added X")
- Include **migration guides** for breaking changes
- Link to **documentation** for new features
- Keep **technical jargon** minimal

## Versioning Strategy

We follow [Semantic Versioning](https://semver.org/):

- **Major (1.0.0):** Breaking changes
- **Minor (0.1.0):** New features (backward-compatible)
- **Patch (0.0.1):** Bug fixes

**Examples:**
- `v0.1.0` → `v0.2.0`: Added new `snps workflows` command
- `v0.2.0` → `v0.2.1`: Fixed crash in `snps daemon`
- `v0.9.0` → `v1.0.0`: Removed deprecated CLI flags

## Testing a Release

Before publishing:

```bash
# Download artifact from draft release
curl -L "https://github.com/HelixoidLLC/pmsynapse/releases/download/v0.2.0/snps-linux-amd64.tar.gz" \
  -o /tmp/snps.tar.gz

# Verify checksum
curl -L "https://github.com/HelixoidLLC/pmsynapse/releases/download/v0.2.0/snps-linux-amd64.tar.gz.sha256" \
  -o /tmp/snps.tar.gz.sha256
sha256sum -c /tmp/snps.tar.gz.sha256

# Extract and test
tar -xzf /tmp/snps.tar.gz -C /tmp
/tmp/snps --version
/tmp/snps --help
```

## Troubleshooting

### Build Fails on One Platform

**Symptom:** Workflow partially succeeds
**Solution:** Workflow continues (fail-fast: false), publish remaining platforms

```bash
# Delete failed tag
git tag -d v0.2.0
git push origin :refs/tags/v0.2.0

# Fix issue, then re-release
./scripts/prepare-release.sh 0.2.0
git push origin main v0.2.0
```

### PAT Expired

**Symptom:** "Resource not accessible by integration" error
**Solution:** Regenerate PAT (see Prerequisites), update secret

### Wrong Version Number

**Symptom:** Release shows old version
**Solution:** Delete draft, fix Cargo.toml, re-tag

```bash
# Delete draft in public repo
gh release delete v0.2.0 --repo HelixoidLLC/pmsynapse --yes

# Delete tag
git tag -d v0.2.0
git push origin :refs/tags/v0.2.0

# Fix and retry
vim Cargo.toml
git commit -am "fix: correct version"
git tag v0.2.0
git push origin main v0.2.0
```

### Binary Too Large

**Symptom:** Archive > 10 MB
**Possible causes:**
- Debug symbols not stripped
- Built in debug mode
**Solution:** Check workflow uses `--release` and `strip` command

## Security Notes

**Source Protection:**
- Binaries contain no source code
- No private repo references in binaries
- Verify with: `strings target/release/snps | grep pmsynapse-mono`

**PAT Security:**
- Use fine-grained tokens (not classic)
- Minimum permissions (contents: write only)
- Rotate annually
- Never commit to repo

**Checksum Verification:**
All releases include SHA256 checksums for integrity verification:
```bash
sha256sum -c snps-linux-amd64.tar.gz.sha256
```

## Rollback a Release

If critical bug found after publishing:

```bash
# 1. Delete the release
gh release delete v0.2.0 --repo HelixoidLLC/pmsynapse --yes

# 2. Delete the tag
git push origin :refs/tags/v0.2.0

# 3. Notify users (create issue in public repo)
gh issue create --repo HelixoidLLC/pmsynapse \
  --title "v0.2.0 Rolled Back" \
  --body "Critical bug found, release removed. Use v0.1.0 instead."

# 4. Fix issue, release as v0.2.1
```

## Reference

**Key Files:**
- `.github/workflows/release-cli.yml` - Release automation
- `.github/release-notes/*.md` - Version-specific release notes
- `scripts/prepare-release.sh` - Release preparation script
- `Cargo.toml` - Workspace version
- `engine/snps-cli/Cargo.toml` - CLI version

**External Links:**
- [softprops/action-gh-release](https://github.com/softprops/action-gh-release)
- [Semantic Versioning](https://semver.org/)
- [GitHub Actions: Publishing packages](https://docs.github.com/en/actions/publishing-packages)
