Feature: Repository CLI Commands
  As a user
  I want to manage repositories via CLI
  So that I can initialize, list, and configure repositories

  Scenario: Repo init command help
    When I run "snps repo init --help"
    Then the command should succeed
    And the output should contain "Initialize new matter repository"
    And the output should contain "--context"
    And the output should contain "--id"

  Scenario: Repo list command help
    When I run "snps repo list --help"
    Then the command should succeed
    And the output should contain "List configured repositories"

  Scenario: Repo add command help
    When I run "snps repo add --help"
    Then the command should succeed
    And the output should contain "Add existing repository"

  Scenario: Repo index command help
    When I run "snps repo index --help"
    Then the command should succeed
    And the output should contain "Rebuild repository index"

  Scenario: Repo init requires arguments
    When I run "snps repo init"
    Then the command should fail
    And the error should contain "required arguments"
