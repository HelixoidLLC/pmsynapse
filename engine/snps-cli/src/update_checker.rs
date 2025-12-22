use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const GITHUB_API_URL: &str = "https://api.github.com/repos/HelixoidLLC/pmsynapse/releases/latest";
const CHECK_INTERVAL_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
struct UpdateCache {
    last_check: DateTime<Utc>,
    latest_version: Option<String>,
    current_version: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    published_at: String,
}

pub struct UpdateChecker {
    cache_path: PathBuf,
    current_version: String,
}

impl UpdateChecker {
    pub fn new() -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine cache directory"))?
            .join("pmsynapse");

        fs::create_dir_all(&cache_dir)?;

        let cache_path = cache_dir.join("update_check.json");
        let current_version = env!("CARGO_PKG_VERSION").to_string();

        Ok(Self {
            cache_path,
            current_version,
        })
    }

    /// Check for updates and return notification message if update available
    pub fn check_and_notify(&self) -> Result<Option<String>> {
        // Check if we should run update check
        if !self.should_check()? {
            return Ok(None);
        }

        // Fetch latest version from GitHub
        let latest_version = self.fetch_latest_version()?;

        // Update cache
        self.update_cache(&latest_version)?;

        // Compare versions and return notification if needed
        if self.is_update_available(&latest_version) {
            Ok(Some(self.format_notification(&latest_version)))
        } else {
            Ok(None)
        }
    }

    fn should_check(&self) -> Result<bool> {
        if !self.cache_path.exists() {
            return Ok(true);
        }

        let cache: UpdateCache = serde_json::from_str(&fs::read_to_string(&self.cache_path)?)?;
        let now = Utc::now();
        let elapsed = now.signed_duration_since(cache.last_check);

        Ok(elapsed > Duration::hours(CHECK_INTERVAL_HOURS))
    }

    fn fetch_latest_version(&self) -> Result<String> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("pmsynapse-cli")
            .timeout(std::time::Duration::from_secs(20))
            .build()?;

        let response = client
            .get(GITHUB_API_URL)
            .header("Accept", "application/vnd.github+json")
            .send()?;

        let status = response.status();

        if status == reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!("No published releases found. Note: Draft/pre-releases are excluded.");
        }

        if !status.is_success() {
            anyhow::bail!("GitHub API request failed: {}", status);
        }

        let release: GitHubRelease = response.json()?;
        Ok(release.tag_name)
    }

    fn update_cache(&self, latest_version: &str) -> Result<()> {
        let cache = UpdateCache {
            last_check: Utc::now(),
            latest_version: Some(latest_version.to_string()),
            current_version: self.current_version.clone(),
        };

        fs::write(&self.cache_path, serde_json::to_string(&cache)?)?;
        Ok(())
    }

    fn is_update_available(&self, latest_version: &str) -> bool {
        // Strip 'v' prefix if present
        let latest = latest_version.trim_start_matches('v');
        let current = self.current_version.trim_start_matches('v');

        // Simple string comparison (could use semver crate for proper version comparison)
        latest != current && latest > current
    }

    fn format_notification(&self, latest_version: &str) -> String {
        format!(
            "\n📢 Update available: v{} → {}\n   Run: curl -fsSL https://github.com/HelixoidLLC/pmsynapse/releases/latest | sh\n",
            self.current_version,
            latest_version
        )
    }
}

/// Check for updates silently in background (fire-and-forget)
/// Can be disabled by setting SNPS_DISABLE_UPDATE_CHECK=1
pub fn check_for_updates_background() {
    // Allow users to disable update checks via environment variable
    if std::env::var("SNPS_DISABLE_UPDATE_CHECK").is_ok() {
        return;
    }

    std::thread::spawn(|| {
        if let Ok(checker) = UpdateChecker::new() {
            if let Ok(Some(message)) = checker.check_and_notify() {
                eprintln!("{}", message);
            }
        }
    });
}
