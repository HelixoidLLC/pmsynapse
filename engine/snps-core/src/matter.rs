//! Matter types and file format handling
//!
//! This module defines the core matter types for PMSynapse's file-based knowledge system.
//! Matter items are markdown files with YAML frontmatter containing metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod parser;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MatterType {
    Spec,
    Document,
    Brainstorming,
    Mindmap,
    Insight,
    Research,
    Plan,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContextType {
    User,
    Team,
    Project,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Private,
    Shared,
    Public,
}

/// Frontmatter metadata for matter files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatterFrontmatter {
    pub matter_type: MatterType,
    pub title: String,
    pub context_type: ContextType,
    pub context_id: String,
    pub visibility: Visibility,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub version: Option<u32>,
    pub status: Option<String>,
    pub folder_path: Option<String>,
}

/// Complete matter item with content
#[derive(Debug, Clone)]
pub struct MatterItem {
    pub frontmatter: MatterFrontmatter,
    pub content: String,
    pub file_path: PathBuf,
}

impl MatterItem {
    /// Parse a matter file from disk
    pub fn parse_file(path: &PathBuf) -> Result<Self, crate::SynapseError> {
        let file_content = std::fs::read_to_string(path)?;

        let (frontmatter, content) = parser::parse_frontmatter(&file_content)?;

        Ok(Self {
            frontmatter,
            content: content.to_string(),
            file_path: path.clone(),
        })
    }

    /// Convert matter item to file content (frontmatter + content)
    pub fn to_file_content(&self) -> String {
        let frontmatter_yaml = serde_yaml::to_string(&self.frontmatter)
            .expect("Failed to serialize frontmatter");

        format!("---\n{}---\n\n{}", frontmatter_yaml, self.content)
    }

    /// Save matter item to disk
    pub fn save(&self) -> Result<(), crate::SynapseError> {
        let content = self.to_file_content();

        // Ensure parent directory exists
        if let Some(parent) = self.file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&self.file_path, content)?;
        Ok(())
    }
}

/// Generate file path for new matter item
pub fn generate_matter_path(
    repo_path: &std::path::Path,
    matter_type: &MatterType,
    title: &str,
    visibility: &Visibility,
) -> PathBuf {
    // Convert title to slug (lowercase, replace spaces with hyphens)
    let slug = title
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();

    // Get date prefix
    let date = chrono::Local::now().format("%Y-%m-%d");

    // Determine directory based on visibility and matter type
    let dir = match visibility {
        Visibility::Private => "private",
        Visibility::Shared => "shared",
        Visibility::Public => "public",
    };

    let type_dir = match matter_type {
        MatterType::Spec => "specs",
        MatterType::Document => "documents",
        MatterType::Brainstorming => "brainstorming",
        MatterType::Mindmap => "mindmaps",
        MatterType::Insight => "insights",
        MatterType::Research => "research",
        MatterType::Plan => "plans",
    };

    let filename = format!("{}-{}.md", date, slug);
    repo_path.join(dir).join(type_dir).join(filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matter_type_serialization() {
        let matter_type = MatterType::Spec;
        let json = serde_json::to_string(&matter_type).unwrap();
        assert_eq!(json, r#""spec""#);

        let deserialized: MatterType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, MatterType::Spec);
    }

    #[test]
    fn test_context_type_serialization() {
        let context = ContextType::Team;
        let json = serde_json::to_string(&context).unwrap();
        assert_eq!(json, r#""team""#);
    }

    #[test]
    fn test_visibility_serialization() {
        let visibility = Visibility::Private;
        let json = serde_json::to_string(&visibility).unwrap();
        assert_eq!(json, r#""private""#);
    }

    #[test]
    fn test_generate_matter_path() {
        let repo = PathBuf::from("/home/user/repo");
        let path = generate_matter_path(
            &repo,
            &MatterType::Spec,
            "Test Specification",
            &Visibility::Shared,
        );

        let path_str = path.to_string_lossy();
        assert!(path_str.contains("/shared/specs/"));
        assert!(path_str.contains("-test-specification.md"));
    }

    #[test]
    fn test_generate_matter_path_sanitizes_title() {
        let repo = PathBuf::from("/home/user/repo");
        let path = generate_matter_path(
            &repo,
            &MatterType::Document,
            "Test!@# Document$%^ 123",
            &Visibility::Private,
        );

        let filename = path.file_name().unwrap().to_string_lossy();
        // Should only contain alphanumeric and hyphens (plus date prefix)
        assert!(filename.contains("test-document-123.md"));
    }
}
