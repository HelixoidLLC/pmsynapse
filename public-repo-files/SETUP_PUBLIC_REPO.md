# Setting Up the Public Repository

This guide explains how to set up `https://github.com/HelixoidLLC/pmsynapse` as the public artifacts repository.

## Initial Setup

### 1. Create the Repository

If not already created:

```bash
# Via GitHub CLI
gh repo create HelixoidLLC/pmsynapse --public --description "PMSynapse - AI-enabled project management CLI"

# Or via GitHub web interface
# https://github.com/organizations/HelixoidLLC/repositories/new
```

### 2. Clone and Initialize

```bash
git clone https://github.com/HelixoidLLC/pmsynapse.git
cd pmsynapse
```

### 3. Copy Files from Source Repository

From the source repository (`pmsynapse-mono`), copy these files to the public repo:

```bash
# From pmsynapse-mono/public-repo-files/ to pmsynapse/
cp public-repo-files/README.md .
cp public-repo-files/install.sh .
cp public-repo-files/install.ps1 .
```

### 4. Create LICENSE File

```bash
cat > LICENSE << 'EOF'
Proprietary License

Copyright (c) 2024-2025 Helixoid LLC. All rights reserved.

This software and associated documentation files (the "Software") are
proprietary and confidential to Helixoid LLC.

The binaries distributed via this repository are provided for use under
the following conditions:

1. The Software may be used for evaluation and production purposes.
2. The Software may not be modified, reverse-engineered, or redistributed
   without explicit written permission from Helixoid LLC.
3. All intellectual property rights remain with Helixoid LLC.

For licensing inquiries, contact: contact@helixoid.com

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
EOF
```

**Note:** Adjust license as needed for your business model.

### 5. Make Install Scripts Executable

```bash
chmod +x install.sh
```

### 6. Commit Initial Files

```bash
git add .
git commit -m "Initial setup: README, installers, license"
git push origin main
```

### 7. Configure Repository Settings

#### Releases
1. Go to Settings → General
2. Under "Features", ensure **Releases** is enabled

#### Issues (Optional)
If you want users to report issues here:
1. Enable Issues in Settings
2. Create issue templates for bug reports, feature requests

#### GitHub Pages (Optional)
For a download landing page:
1. Settings → Pages
2. Source: Deploy from a branch
3. Branch: `main` / `docs` folder
4. Create `docs/index.html` with download page

### 8. Test Install Scripts

Before the first release, test the installer structure:

```bash
# Test install.sh URL is accessible
curl -I https://raw.githubusercontent.com/HelixoidLLC/pmsynapse/main/install.sh
# Should return: HTTP/2 200

# Test install.ps1 URL
curl -I https://raw.githubusercontent.com/HelixoidLLC/pmsynapse/main/install.ps1
# Should return: HTTP/2 200
```

## Repository Structure

After setup, the public repo should look like:

```
pmsynapse/
├── README.md              # Installation instructions, getting started
├── LICENSE                # Proprietary license
├── install.sh             # Linux/macOS installer
├── install.ps1            # Windows installer
└── .github/
    └── ISSUE_TEMPLATE/    # Optional: bug report template
```

**What it does NOT contain:**
- No source code
- No build configurations
- No development files
- Only releases and documentation

## First Release Test

After the first release is published from the private repo:

### Verify Release Page
```bash
open "https://github.com/HelixoidLLC/pmsynapse/releases/latest"
```

Should show:
- Version tag (e.g., v0.1.0)
- Release notes
- 10 downloadable files (5 archives + 5 checksums)

### Test Download Links

```bash
# Test direct download URL
curl -I "https://github.com/HelixoidLLC/pmsynapse/releases/latest/download/snps-linux-amd64.tar.gz"
# Should return: HTTP/2 302 (redirect to actual file)
```

### Test Installer

```bash
# Linux/macOS
bash <(curl -fsSL https://raw.githubusercontent.com/HelixoidLLC/pmsynapse/main/install.sh)

# Windows (PowerShell)
irm https://raw.githubusercontent.com/HelixoidLLC/pmsynapse/main/install.ps1 | iex
```

## Maintenance

### Updating Documentation

```bash
# Update README or installers
vim README.md
git commit -am "docs: update installation instructions"
git push origin main
```

Changes are immediate (no release needed).

### Monitoring Downloads

GitHub provides download statistics:
1. Go to Releases
2. Click on a release
3. Scroll to bottom to see download counts per asset

### Deleting Old Releases

To remove outdated versions:

```bash
# Via GitHub CLI
gh release delete v0.1.0 --repo HelixoidLLC/pmsynapse --yes

# Or via web interface
# Releases → Click release → Delete release
```

**Note:** Keep at least the last 2-3 releases for users who haven't upgraded.

## Security Considerations

### Repository Visibility
- **Public** - Anyone can see and download releases
- No source code is exposed
- Consider adding a security policy (`.github/SECURITY.md`)

### Access Control
- Limit write access to trusted team members
- Only the `PUBLIC_REPO_TOKEN` from private repo can create releases
- Consider branch protection on `main`

### Release Signatures (Future)
For enhanced security, consider GPG-signing releases:
- Generate GPG key
- Sign archives: `gpg --detach-sign snps-linux-amd64.tar.gz`
- Include `.sig` files in release
- Document verification in README

## Troubleshooting

### Install Script Returns 404

**Cause:** File not in `main` branch
**Fix:**
```bash
git checkout main
git pull origin main
# Verify file exists: ls install.sh
```

### Release Not Showing as "Latest"

**Cause:** Release marked as pre-release or draft
**Fix:**
1. Edit release
2. Uncheck "Set as a pre-release"
3. Ensure "Draft" is unchecked
4. Update release

### Installer Downloads Wrong Version

**Cause:** `/latest` points to wrong release
**Fix:**
- Only publish stable releases as non-draft, non-prerelease
- Beta releases should be marked as "pre-release"

## Next Steps

After initial setup:
1. Test full release workflow from private repo
2. Verify installers work on all platforms
3. Create landing page (optional)
4. Announce first release
5. Set up analytics (optional)

## Support

If users encounter issues:
- Direct them to email: support@helixoid.com
- Or enable GitHub Issues for public bug reports
