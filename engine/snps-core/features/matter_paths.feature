Feature: Matter Path Generation
  As a developer
  I want to generate correct file paths for Matter items
  So that files are organized properly in the repository

  Scenario: Generate path for spec in shared directory
    Given matter_type "spec" and visibility "shared"
    And title "Test Specification"
    When I generate the path
    Then the path should contain "/shared/specs/"
    And the filename should end with "test-specification.md"

  Scenario: Generate path sanitizes title with special characters
    Given matter_type "document" and visibility "private"
    And title "Test!@# Document$%^ 123"
    When I generate the path
    Then the path should contain "/private/documents/"
    And the filename should contain "test-document-123.md"

  Scenario: Slugify title correctly
    Given matter_type "research" and visibility "public"
    And title "API Research Report"
    When I generate the path
    Then the filename should end with "api-research-report.md"
