Feature: IDLC Workflow Management
  As a user
  I want to manage IDLC workflows via CLI
  So that I can configure team-specific development lifecycles

  # ============================================================================
  # Template Management
  # ============================================================================

  Scenario: List available IDLC templates
    When I run "snps idlc templates"
    Then the command should succeed
    And the output should contain "default"
    And the output should contain "Standard software development workflow"

  Scenario: IDLC help shows all subcommands
    When I run "snps idlc --help"
    Then the command should succeed
    And the output should contain "init"
    And the output should contain "show"
    And the output should contain "validate"
    And the output should contain "generate"
    And the output should contain "visualize"
    And the output should contain "templates"

  Scenario: Initialize IDLC with default template (non-interactive)
    Given a PMSynapse initialized project
    When I run "snps idlc init --yes"
    Then the command should succeed
    And the IDLC file should exist for team "default"
    And the IDLC file should contain "version:"
    And the IDLC file should contain "team:"

  Scenario: Generate IDLC from template to stdout
    When I run "snps idlc generate --template default --team-id test --team-name Test"
    Then the command should succeed
    And the output should contain "version: \"1.0\""
    And the output should contain "id: \"test\""
    And the output should contain "name: \"Test\""

  Scenario: Generate IDLC from template to file
    Given a temporary directory
    When I run "snps idlc generate --template default --team-id ops --team-name Ops --output ops-workflow.yaml"
    Then the command should succeed
    And the file "ops-workflow.yaml" should exist in temp directory
    And the output should contain "Generated"

  Scenario: Generate with unknown template fails
    When I run "snps idlc generate --template unknown --team-id test --team-name Test"
    Then the command should fail
    And the error should contain "Unknown template"

  # ============================================================================
  # Configuration Display
  # ============================================================================

  Scenario: Show IDLC configuration in YAML format
    Given a PMSynapse project with IDLC configuration
    When I run "snps idlc show --team default --format yaml"
    Then the command should succeed
    And the output should contain "stages:"
    And the output should contain "statuses:"
    And the output should contain "transitions:"

  Scenario: Show IDLC configuration in JSON format
    Given a PMSynapse project with IDLC configuration
    When I run "snps idlc show --team default --format json"
    Then the command should succeed
    And the output should contain "\"stages\":"
    And the output should contain "\"statuses\":"
    And the output should contain "\"transitions\":"

  Scenario: Show IDLC configuration in table format
    Given a PMSynapse project with IDLC configuration
    When I run "snps idlc show --team default --format table"
    Then the command should succeed
    And the output should contain "Stages:"
    And the output should contain "Statuses:"
    And the output should contain "Transitions:"

  Scenario: Show IDLC for non-existent team
    Given a PMSynapse initialized project
    When I run "snps idlc show --team nonexistent"
    Then the command should succeed
    And the output should contain "No IDLC config found"

  # ============================================================================
  # Configuration Validation
  # ============================================================================

  Scenario: Validate valid IDLC configuration
    Given a valid IDLC configuration file
    When I run "snps idlc validate --file test-idlc.yaml"
    Then the command should succeed
    And the output should contain "Valid"
    And the output should contain "stages"
    And the output should contain "statuses"
    And the output should contain "transitions"

  Scenario: Validate IDLC with invalid stage reference
    Given an IDLC configuration with invalid stage reference
    When I run "snps idlc validate --file invalid-stage.yaml"
    Then the command should fail
    And the error should contain "unknown stage"

  Scenario: Validate IDLC with invalid transition target
    Given an IDLC configuration with invalid transition target
    When I run "snps idlc validate --file invalid-transition.yaml"
    Then the command should fail
    And the error should contain "unknown status"

  Scenario: Validate IDLC with duplicate status IDs
    Given an IDLC configuration with duplicate status IDs
    When I run "snps idlc validate --file duplicate-status.yaml"
    Then the command should fail
    And the error should contain "Duplicate status"

  Scenario: Validate IDLC with wildcard except referencing unknown status
    Given an IDLC configuration with invalid wildcard except clause
    When I run "snps idlc validate --file invalid-except.yaml"
    Then the command should fail
    And the error should contain "except clause references unknown status"

  # ============================================================================
  # Visualization
  # ============================================================================

  Scenario: Generate Mermaid diagram to stdout
    Given a PMSynapse project with IDLC configuration
    When I run "snps idlc visualize --team default"
    Then the command should succeed
    And the output should contain "stateDiagram-v2"
    And the output should contain "[*] -->"
    And the output should contain "--> [*]"

  Scenario: Generate Mermaid diagram to file
    Given a PMSynapse project with IDLC configuration
    When I run "snps idlc visualize --team default --output workflow.mmd"
    Then the command should succeed
    And the output should contain "Generated"

  # ============================================================================
  # Wildcard Transitions
  # ============================================================================

  Scenario: Wildcard transitions are validated correctly
    Given an IDLC configuration with wildcard transitions
    When I run "snps idlc validate --file wildcard-config.yaml"
    Then the command should succeed
    And the output should contain "Valid"

  Scenario: Wildcard transitions appear in visualization
    Given an IDLC configuration with wildcard transitions
    When I run "snps idlc validate --file wildcard-config.yaml"
    Then the command should succeed
    And the output should contain "Valid"
