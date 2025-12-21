//! Built-in IDLC workflow templates

/// Default software development workflow
pub const DEFAULT_TEMPLATE: &str = include_str!("templates/default.yaml");

/// Get template by name
pub fn get_template(name: &str) -> Option<&'static str> {
    match name {
        "default" => Some(DEFAULT_TEMPLATE),
        _ => None,
    }
}

/// List available template names
pub fn list_templates() -> Vec<(&'static str, &'static str)> {
    vec![("default", "Standard software development workflow")]
}
