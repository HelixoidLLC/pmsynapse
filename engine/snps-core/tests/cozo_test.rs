use cozo::{DbInstance, ScriptMutability};
use tempfile::tempdir;

#[test]
fn test_cozo_basic_functionality() -> Result<(), Box<dyn std::error::Error>> {
    // Create SQLite database in a temporary directory
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test.db");
    let db = DbInstance::new("sqlite", db_path.to_str().unwrap(), "")?;

    // Create a simple relation and query it
    let result = db.run_script(
        "?[a, b] <- [[1, 2], [3, 4]]",
        Default::default(),
        ScriptMutability::Immutable,
    )?;

    // Verify we got a result
    assert!(result.rows.len() == 2, "Expected 2 rows");

    println!("CozoDB integration test passed!");
    println!("Result: {:?}", result);

    Ok(())
}

#[test]
fn test_cozo_datalog_operations() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test_datalog.db");
    let db = DbInstance::new("sqlite", db_path.to_str().unwrap(), "")?;

    // Test datalog rules and joins
    let result = db.run_script(
        r#"
        data[a, b] <- [[1, 2], [2, 3], [3, 4]]
        data2[b, c] <- [[2, 100], [3, 200], [4, 300]]
        ?[a, b, c] := data[a, b], data2[b, c]
        "#,
        Default::default(),
        ScriptMutability::Immutable,
    )?;

    // Should have 3 joined results
    assert_eq!(result.rows.len(), 3, "Expected 3 joined results");

    println!("CozoDB datalog operations test passed!");
    println!("Joined results: {:?}", result);

    Ok(())
}
