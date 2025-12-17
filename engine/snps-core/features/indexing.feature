Feature: Matter Indexing
  As a developer
  I want to index Matter files in a database
  So that I can search and retrieve them efficiently

  Scenario: Create index with database
    Given a valid database path
    When I create a MatterIndex
    Then the index should be created successfully

  Scenario: Index matter file
    Given a MatterIndex with a database
    And a matter file with title "Test Document"
    When I index the file
    Then the file should be indexed successfully

  Scenario: Search indexed files
    Given a MatterIndex with indexed files
      | title                  |
      | API Specification      |
      | API Design Document    |
      | User Guide             |
    When I search for "api"
    Then I should get 2 results
