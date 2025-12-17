Feature: Basic CLI Functionality
  As a user
  I want basic CLI commands to work
  So that I can access help and version information

  Scenario: CLI help command
    When I run "snps --help"
    Then the command should succeed
    And the output should contain "PMSynapse CLI"
    And the output should contain "matter"
    And the output should contain "repo"
    And the output should contain "config"

  Scenario: CLI version command
    When I run "snps --version"
    Then the command should succeed
    And the output should contain "snps"

  Scenario: Matter subcommand exists
    When I run "snps matter --help"
    Then the command should succeed
    And the output should contain "matter"
    And the output should contain "create"
    And the output should contain "list"
    And the output should contain "search"

  Scenario: Repo subcommand exists
    When I run "snps repo --help"
    Then the command should succeed
    And the output should contain "repo"
    And the output should contain "init"
    And the output should contain "list"
    And the output should contain "add"

  Scenario: Config show command help
    When I run "snps config show --help"
    Then the command should succeed
    And the output should contain "Show merged configuration"
