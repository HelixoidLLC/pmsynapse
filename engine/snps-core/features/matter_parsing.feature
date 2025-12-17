Feature: Matter File Parsing
  As a developer
  I want to parse Matter protocol markdown files
  So that I can extract frontmatter and content correctly

  Scenario: Parse valid frontmatter
    Given a markdown file with valid frontmatter
      | field        | value                 |
      | matter_type  | spec                  |
      | title        | Test Specification    |
      | context_type | user                  |
      | context_id   | igor                  |
      | visibility   | private               |
      | created_by   | igor                  |
    When I parse the file
    Then the frontmatter should be parsed correctly
    And the content should be "This is the document content."

  Scenario: Reject file without frontmatter delimiter
    Given a markdown file without frontmatter delimiter
    When I try to parse the file
    Then parsing should fail
    And the error should contain "Content does not start with frontmatter delimiter"

  Scenario: Reject file with unclosed frontmatter
    Given a markdown file with unclosed frontmatter
    When I try to parse the file
    Then parsing should fail
    And the error should contain "Could not find closing frontmatter delimiter"
