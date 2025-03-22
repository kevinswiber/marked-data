// Tests for duplicate key detection in flow mappings

use marked_yaml::{parse_yaml_with_options, LoaderOptions, LoadError};

#[test]
fn simple_flow_mapping_duplicate_keys() {
    // Test basic flow mapping with duplicate keys
    let yaml = "{foo: bar, foo: baz}";
    
    // With error_on_duplicate_keys = true, we should get an error
    let result = parse_yaml_with_options(
        0,
        yaml,
        LoaderOptions::default().error_on_duplicate_keys(true),
    );
    
    assert!(result.is_err());
    if let Err(LoadError::DuplicateKey(inner)) = result {
        assert_eq!(inner.prev_key.as_str(), "foo");
        assert_eq!(inner.key.as_str(), "foo");
    } else {
        panic!("Expected DuplicateKey error, got {:?}", result);
    }
    
    // With error_on_duplicate_keys = false, the last key should win
    let result = parse_yaml_with_options(
        0,
        yaml,
        LoaderOptions::default().error_on_duplicate_keys(false),
    );
    
    assert!(result.is_ok());
    let node = result.unwrap();
    let map = node.as_mapping().unwrap();
    
    // Check if the key exists and has the expected value
    let foo_value = map.get("foo");
    assert!(foo_value.is_some(), "Expected key 'foo' to exist in the mapping");
    
    // Check that the value is "baz" (the last value for the duplicate key)
    match foo_value.unwrap() {
        marked_yaml::Node::Scalar(scalar) => {
            assert_eq!(scalar.as_str(), "baz", "Expected value to be 'baz'");
        },
        _ => panic!("Expected 'foo' to map to a scalar value")
    }
}

#[test]
fn multiline_flow_mapping_duplicate_keys() {
    // Test multi-line flow mapping with duplicate keys
    let yaml = "{\n  foo: bar,\n  foo: baz\n}";
    
    // With error_on_duplicate_keys = true, we should get an error
    let result = parse_yaml_with_options(
        0,
        yaml,
        LoaderOptions::default().error_on_duplicate_keys(true),
    );
    
    assert!(result.is_err());
    if let Err(LoadError::DuplicateKey(inner)) = result {
        assert_eq!(inner.prev_key.as_str(), "foo");
        assert_eq!(inner.key.as_str(), "foo");
        
        // Check that line numbers are correct
        assert_eq!(inner.prev_key.span().start().unwrap().line(), 2);
        assert_eq!(inner.key.span().start().unwrap().line(), 3);
    } else {
        panic!("Expected DuplicateKey error, got {:?}", result);
    }
}

#[test]
fn complex_flow_mapping_duplicate_keys() {
    // Test flow mapping with nested structures and duplicate keys
    let yaml = "{\n  foo: {nested: value},\n  bar: [1, 2, 3],\n  foo: final value\n}";
    
    // With error_on_duplicate_keys = true, we should get an error
    let result = parse_yaml_with_options(
        0,
        yaml,
        LoaderOptions::default().error_on_duplicate_keys(true),
    );
    
    assert!(result.is_err());
    if let Err(LoadError::DuplicateKey(inner)) = result {
        assert_eq!(inner.prev_key.as_str(), "foo");
        assert_eq!(inner.key.as_str(), "foo");
        
        // Check that line numbers are correct
        assert_eq!(inner.prev_key.span().start().unwrap().line(), 2);
        assert_eq!(inner.key.span().start().unwrap().line(), 4);
    } else {
        panic!("Expected DuplicateKey error, got {:?}", result);
    }
    
    // With error_on_duplicate_keys = false, the last key should win
    let result = parse_yaml_with_options(
        0,
        yaml,
        LoaderOptions::default().error_on_duplicate_keys(false),
    );
    
    assert!(result.is_ok());
    let node = result.unwrap();
    let map = node.as_mapping().unwrap();
    
    // Check if the key exists and has the expected value
    let foo_value = map.get("foo");
    assert!(foo_value.is_some(), "Expected key 'foo' to exist in the mapping");
    
    // Check that the value is "final value" (the last value for the duplicate key)
    match foo_value.unwrap() {
        marked_yaml::Node::Scalar(scalar) => {
            assert_eq!(scalar.as_str(), "final value", "Expected value to be 'final value'");
        },
        _ => panic!("Expected 'foo' to map to a scalar value")
    }
}

#[test]
fn quoted_keys_flow_mapping_duplicate_keys() {
    // Test flow mapping with quoted keys
    let yaml = "{\n  \"foo\": bar,\n  \"foo\": baz\n}";
    
    // With error_on_duplicate_keys = true, we should get an error
    let result = parse_yaml_with_options(
        0,
        yaml,
        LoaderOptions::default().error_on_duplicate_keys(true),
    );
    
    assert!(result.is_err());
    if let Err(LoadError::DuplicateKey(inner)) = result {
        assert_eq!(inner.prev_key.as_str(), "foo");
        assert_eq!(inner.key.as_str(), "foo");
    } else {
        panic!("Expected DuplicateKey error, got {:?}", result);
    }
}
