Feature: Matter CLI Commands
  As a user
  I want to manage Matter items via CLI
  So that I can create, list, and search documents

  Scenario: Matter list command help
    When I run "snps matter list --help"
    Then the command should succeed
    And the output should contain "List matter items"
    And the output should contain "--context"

  Scenario: Matter search command help
    When I run "snps matter search --help"
    Then the command should succeed
    And the output should contain "Search matter items"
    And the output should contain "query"

  Scenario: Matter create requires arguments
    When I run "snps matter create"
    Then the command should fail
    And the error should contain "required arguments"

  Scenario: Matter create command with full arguments
    Given a temporary repository directory
    When I run matter create with all required arguments
    Then the command should succeed or document expected behavior
