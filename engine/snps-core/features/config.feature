Feature: Configuration Management
  As a developer
  I want to manage configuration settings
  So that the system behaves according to user preferences

  Scenario: Default config has expected values
    When I create a default config
    Then the version should be "1.0"
    And the default matter_type should be "document"
    And the default visibility should be "private"
    And index_enabled should be true
    And auto_sync should be false

  Scenario: Config serialization round trip
    Given a default config
    When I serialize and deserialize the config
    Then the deserialized config should match the original

  Scenario: Merged config tracks sources
    When I load merged config with no overrides
    Then config sources should be tracked
    And "version" should have source "Default"
    And "defaults.editor" should have source "Default"
