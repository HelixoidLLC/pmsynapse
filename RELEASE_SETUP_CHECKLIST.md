# PMSynapse Release Pipeline - Setup Checklist

Complete these steps to enable cross-repo releases from the private source repo to the public artifacts repo.

## Repository Configuration

### Source Repository: `HelixoidLLC/pmsynapse-mono` (Private)

- [x] Workflow created: `.github/workflows/release-cli.yml`
- [x] Release notes template: `.github/release-notes/TEMPLATE.md`
- [x] Release script: `scripts/prepare-release.sh` (executable)
- [x] Documentation: `docs/RELEASE_PROCESS.md`
- [ ] **TODO: Create GitHub PAT (Personal Access Token)**
  - Go to: Settings → Developer settings → Personal access tokens → Fine-grained tokens
  - Name: "PMSynapse Release Publisher"
  - Repository access: Only `HelixoidLLC/pmsynapse`
  - Permissions: Contents (read & write)
  - Expiration: 1 year
- [ ] **TODO: Add PAT as repository secret**
  - Repo Settings → Secrets and variables → Actions
  - Name: `PUBLIC_REPO_TOKEN`
  - Value: [paste PAT from above]

### Public Repository: `HelixoidLLC/pmsynapse` (Public)

- [ ] **TODO: Create repository if needed**
  ```bash
  gh repo create HelixoidLLC/pmsynapse --public --description "PMSynapse - AI-enabled project management CLI"
  ```
- [ ] **TODO: Copy files from `public-repo-files/`**
  ```bash
  cd /path/to/pmsynapse
  cp /path/to/pmsynapse-mono/public-repo-files/README.md .
  cp /path/to/pmsynapse-mono/public-repo-files/install.sh .
  cp /path/to/pmsynapse-mono/public-repo-files/install.ps1 .
  chmod +x install.sh
  git add .
  git commit -m "Initial setup: installers and documentation"
  git push origin main
  ```
- [ ] **TODO: Add LICENSE file**
  - See `public-repo-files/SETUP_PUBLIC_REPO.md` for template
- [ ] **TODO: Enable Releases in repo settings**
  - Settings → General → Features → ✓ Releases

## Testing the Pipeline

### 1. Test Release (Dry Run)

- [ ] Create test tag in source repo:
  ```bash
  cd /path/to/pmsynapse-mono
  git tag v0.0.1-test
  git push origin v0.0.1-test
  ```
- [ ] Monitor workflow: `https://github.com/HelixoidLLC/pmsynapse-mono/actions`
- [ ] Verify draft release created: `https://github.com/HelixoidLLC/pmsynapse/releases`
- [ ] Check all 10 files attached (5 archives + 5 checksums)
- [ ] Delete test release:
  ```bash
  gh release delete v0.0.1-test --repo HelixoidLLC/pmsynapse --yes
  git tag -d v0.0.1-test
  git push origin :refs/tags/v0.0.1-test
  ```

### 2. Test Installers

- [ ] Test Linux installer:
  ```bash
  bash <(curl -fsSL https://raw.githubusercontent.com/HelixoidLLC/pmsynapse/main/install.sh)
  ```
- [ ] Test macOS installer (same command as Linux)
- [ ] Test Windows installer:
  ```powershell
  irm https://raw.githubusercontent.com/HelixoidLLC/pmsynapse/main/install.ps1 | iex
  ```

## First Production Release

- [ ] Create release notes: `.github/release-notes/v0.1.0.md`
  - Template already created, review and customize
- [ ] Run release script:
  ```bash
  ./scripts/prepare-release.sh 0.1.0
  ```
- [ ] Review and push:
  ```bash
  git show HEAD  # Review version bump commit
  git push origin main
  git push origin v0.1.0
  ```
- [ ] Monitor workflow completion (~5-10 min)
- [ ] Review draft release in public repo
- [ ] Test download one artifact manually
- [ ] **Publish release** (click button in GitHub UI)
- [ ] Verify installer works end-to-end

## Post-Release

- [ ] Announce release (email, social, etc.)
- [ ] Update documentation if needed
- [ ] Monitor download counts in release page

## Files Created

### Source Repo (`pmsynapse-mono`)
```
.github/
├── workflows/
│   └── release-cli.yml              # Main release workflow
└── release-notes/
    ├── TEMPLATE.md                  # Template for release notes
    └── v0.1.0.md                    # Example release notes

scripts/
└── prepare-release.sh               # Helper script for releases

docs/
├── RELEASE_PROCESS.md               # Complete release guide
└── RELEASE_SETUP_CHECKLIST.md       # This file

public-repo-files/                   # Files to copy to public repo
├── README.md                        # Public repo README
├── install.sh                       # Linux/macOS installer
├── install.ps1                      # Windows installer
└── SETUP_PUBLIC_REPO.md             # Public repo setup guide
```

### Public Repo (`pmsynapse`) - After Setup
```
README.md                            # Installation & getting started
install.sh                           # Linux/macOS installer
install.ps1                          # Windows installer
LICENSE                              # Software license
```

## Troubleshooting

### "Resource not accessible by integration"
- **Cause:** PAT not set or expired
- **Fix:** Create/update `PUBLIC_REPO_TOKEN` secret

### Workflow builds but doesn't create release
- **Cause:** PAT lacks `contents: write` permission
- **Fix:** Regenerate PAT with correct permissions

### Release shows old version
- **Cause:** Forgot to bump version in Cargo.toml
- **Fix:** Use `prepare-release.sh` script

### Installer returns 404
- **Cause:** Files not in public repo `main` branch
- **Fix:** Copy files from `public-repo-files/` and commit

## Next Steps

After completing this checklist:
1. Read `docs/RELEASE_PROCESS.md` for ongoing release procedures
2. Consider adding Desktop app releases (future)
3. Set up changelog automation (optional)
4. Create download landing page (optional)

## Support

Questions? Check:
- `docs/RELEASE_PROCESS.md` - Detailed release guide
- `public-repo-files/SETUP_PUBLIC_REPO.md` - Public repo setup
- `.github/workflows/release-cli.yml` - Workflow reference
