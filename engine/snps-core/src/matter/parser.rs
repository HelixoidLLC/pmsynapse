//! YAML frontmatter parsing for matter files

use super::MatterFrontmatter;

/// Parse YAML frontmatter from markdown content
///
/// Expects frontmatter to be delimited by `---` at the start and end
/// Returns (frontmatter, remaining_content)
pub fn parse_frontmatter(content: &str) -> Result<(MatterFrontmatter, &str), crate::SynapseError> {
    // Check if content starts with ---
    if !content.starts_with("---") {
        return Err(crate::SynapseError::Matter(
            "Content does not start with frontmatter delimiter '---'".to_string(),
        ));
    }

    // Find the closing ---
    let rest = &content[3..]; // Skip first ---
    let Some(end_pos) = rest.find("\n---") else {
        return Err(crate::SynapseError::Matter(
            "Could not find closing frontmatter delimiter '---'".to_string(),
        ));
    };

    let frontmatter_str = &rest[..end_pos].trim();
    let remaining_content = &rest[end_pos + 4..].trim(); // Skip \n---

    // Parse YAML
    let frontmatter: MatterFrontmatter =
        serde_yaml::from_str(frontmatter_str).map_err(crate::SynapseError::YamlParse)?;

    Ok((frontmatter, remaining_content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matter::{ContextType, MatterType, Visibility};
    use chrono::Utc;

    #[test]
    fn test_parse_valid_frontmatter() {
        let content = r#"---
matter_type: spec
title: Test Spec
context_type: user
context_id: igor
visibility: private
tags:
  - test
  - spec
created_at: '2024-01-01T00:00:00Z'
created_by: igor
---

This is the content of the spec."#;

        let result = parse_frontmatter(content);
        assert!(result.is_ok());

        let (frontmatter, remaining) = result.unwrap();
        assert_eq!(frontmatter.title, "Test Spec");
        assert_eq!(frontmatter.matter_type, MatterType::Spec);
        assert_eq!(frontmatter.context_type, ContextType::User);
        assert_eq!(frontmatter.visibility, Visibility::Private);
        assert_eq!(frontmatter.tags, vec!["test", "spec"]);
        assert_eq!(remaining, "This is the content of the spec.");
    }

    #[test]
    fn test_parse_missing_delimiter() {
        let content = "No frontmatter here";
        let result = parse_frontmatter(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_closing_delimiter() {
        let content = r#"---
matter_type: spec
title: Test
"#;
        let result = parse_frontmatter(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_round_trip() {
        use crate::matter::{MatterFrontmatter, MatterItem};
        use std::path::PathBuf;

        let frontmatter = MatterFrontmatter {
            matter_type: MatterType::Document,
            title: "Round Trip Test".to_string(),
            context_type: ContextType::User,
            context_id: "test".to_string(),
            visibility: Visibility::Private,
            tags: vec!["test".to_string()],
            created_at: Utc::now(),
            created_by: "test".to_string(),
            updated_at: None,
            updated_by: None,
            version: Some(1),
            status: None,
            folder_path: None,
        };

        let item = MatterItem {
            frontmatter: frontmatter.clone(),
            content: "Test content".to_string(),
            file_path: PathBuf::from("/tmp/test.md"),
        };

        let file_content = item.to_file_content();
        let (parsed_frontmatter, parsed_content) = parse_frontmatter(&file_content).unwrap();

        assert_eq!(parsed_frontmatter.title, frontmatter.title);
        assert_eq!(parsed_frontmatter.matter_type, frontmatter.matter_type);
        assert_eq!(parsed_content, "Test content");
    }
}
