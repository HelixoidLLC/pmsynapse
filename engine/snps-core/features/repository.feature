Feature: Repository Configuration
  As a developer
  I want to configure repository settings
  So that I can control sync and context behavior

  Scenario: Default sync config has expected values
    When I create a default sync config
    Then sync enabled should be false
    And the branch should be "main"
    And remote should be none

  Scenario: Repository context type serialization
    Given a ContextType "team"
    When I serialize it to JSON
    Then it should serialize to "team"
    And deserialize correctly

  Scenario: Repository visibility serialization
    Given a Visibility "shared"
    When I serialize it to JSON
    Then it should serialize to "shared"
