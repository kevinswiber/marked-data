use marked_yaml::{parse_yaml, parse_yaml_with_options, LoaderOptions};
use std::fmt::Write;

fn visualize_position(input: &str, line: usize, column: usize) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut result = String::new();

    if line <= lines.len() {
        let target_line = lines[line - 1];
        writeln!(result, "Line {}: \"{}\"", line, target_line).unwrap();

        if column <= target_line.len() + 1 {
            // The +8 accounts for "Line X: " prefix (8 chars for single digit line numbers)
            let offset = if line < 10 { 8 } else { 9 };

            // Build the pointer line with spaces and a caret
            let mut pointer = " ".repeat(offset);
            pointer.push_str(&" ".repeat(column));
            pointer.push('^');

            writeln!(result, "{}", pointer).unwrap();
        } else {
            writeln!(
                result,
                "{} (column {} is out of bounds for this line)",
                " ".repeat(8 + target_line.len()) + "^",
                column
            )
            .unwrap();
        }
    } else {
        writeln!(result, "Line {} is out of bounds for the input", line).unwrap();
    }

    result
}

#[test]
fn sequence_of_mappings_spans() {
    // This YAML represents a sequence of mappings with specific indentation and formatting
    // that we'll use to test span accuracy
    let yaml = r#"---
- name: first item
  value: 123
  desc: a simple value
- name: second item
  value: 456
  nested:
    key: inner value
    another: thing
- name: third item
  value: 789
  list:
    - list item 1
    - list item 2
"#;

    println!("\n=== YAML Document ===\n{}", yaml);

    // Parse the YAML document - we need to use parse_yaml_with_options since the top level is a sequence
    let node =
        parse_yaml_with_options(0, yaml, LoaderOptions::default().toplevel_sequence()).unwrap();

    // It should be a sequence
    let sequence = node.as_sequence().unwrap();

    // The sequence should have a valid start marker - expected to point to the first dash
    assert!(sequence.span().start().is_some());
    let seq_start = sequence.span().start().unwrap();
    println!("\n=== Sequence Start ===");
    println!("Expected: line 2, column 1");
    println!(
        "Actual: line {}, column {}",
        seq_start.line(),
        seq_start.column()
    );
    assert_eq!(seq_start.line(), 2);
    assert_eq!(seq_start.column(), 1);
    println!(
        "{}",
        visualize_position(yaml, seq_start.line(), seq_start.column())
    );

    // Get the first item from the sequence (which is a mapping)
    let first_item = sequence.get_mapping(0).unwrap();

    // First item should start at the first key name (name), not the dash
    assert!(first_item.span().start().is_some());
    let item_start = first_item.span().start().unwrap();
    println!("\n=== First Item Start ===");
    println!("Expected: line 2, column 7"); // This points to the 'n' in "name"
    println!(
        "Actual: line {}, column {}",
        item_start.line(),
        item_start.column()
    );
    assert_eq!(item_start.line(), 2);
    assert_eq!(item_start.column(), 7);
    println!(
        "{}",
        visualize_position(yaml, item_start.line(), item_start.column())
    );

    // "name" key in first item should be on line 2, column 3
    let name_key = first_item.keys().find(|k| k.as_str() == "name").unwrap();
    assert!(name_key.span().start().is_some());
    let name_key_start = name_key.span().start().unwrap();
    println!("\n=== 'name' Key Start ===");
    println!("Expected: line 2, column 3");
    println!(
        "Actual: line {}, column {}",
        name_key_start.line(),
        name_key_start.column()
    );
    assert_eq!(name_key_start.line(), 2);
    assert_eq!(name_key_start.column(), 3);
    println!(
        "{}",
        visualize_position(yaml, name_key_start.line(), name_key_start.column())
    );

    // "value" key in first item should be on line 3, column 3
    let value_key = first_item.keys().find(|k| k.as_str() == "value").unwrap();
    assert!(value_key.span().start().is_some());
    let value_key_start = value_key.span().start().unwrap();
    println!("\n=== 'value' Key Start ===");
    println!("Expected: line 3, column 3");
    println!(
        "Actual: line {}, column {}",
        value_key_start.line(),
        value_key_start.column()
    );
    assert_eq!(value_key_start.line(), 3);
    assert_eq!(value_key_start.column(), 3);
    println!(
        "{}",
        visualize_position(yaml, value_key_start.line(), value_key_start.column())
    );

    // Get the "first item" value and check content and position
    let name_val = first_item.get_scalar("name").unwrap();
    assert_eq!(name_val.as_str(), "first item");
    println!("\n=== 'first item' Value Start ===");
    println!("Expected: line 2, column 9");
    println!(
        "Actual: line {}, column {}",
        name_val.span().start().unwrap().line(),
        name_val.span().start().unwrap().column()
    );
    assert_eq!(name_val.span().start().unwrap().line(), 2);
    assert_eq!(name_val.span().start().unwrap().column(), 9);
    println!(
        "{}",
        visualize_position(
            yaml,
            name_val.span().start().unwrap().line(),
            name_val.span().start().unwrap().column()
        )
    );

    // Get the second item from the sequence
    let second_item = sequence.get_mapping(1).unwrap();

    // Second item should start at the key name, not the dash
    assert!(second_item.span().start().is_some());
    let second_item_start = second_item.span().start().unwrap();
    println!("\n=== Second Item Start ===");
    println!("Expected: line 5, column 7"); // This points to the 'n' in "name"
    println!(
        "Actual: line {}, column {}",
        second_item_start.line(),
        second_item_start.column()
    );
    assert_eq!(second_item_start.line(), 5);
    assert_eq!(second_item_start.column(), 7);
    println!(
        "{}",
        visualize_position(yaml, second_item_start.line(), second_item_start.column())
    );

    // Check that the nested mapping exists in the second item
    let nested = second_item.get_mapping("nested").unwrap();
    println!("\n=== 'nested' Mapping Start ===");
    println!("Expected: line 8, column 8");
    println!(
        "Actual: line {}, column {}",
        nested.span().start().unwrap().line(),
        nested.span().start().unwrap().column()
    );
    assert_eq!(nested.span().start().unwrap().line(), 8);
    assert_eq!(nested.span().start().unwrap().column(), 8);
    println!(
        "{}",
        visualize_position(
            yaml,
            nested.span().start().unwrap().line(),
            nested.span().start().unwrap().column()
        )
    );

    // Check the "key" in the nested mapping
    let nested_keys = nested.keys().collect::<Vec<_>>();
    let key = nested_keys.iter().find(|k| k.as_str() == "key").unwrap();
    println!("\n=== 'key' in nested Start ===");
    println!("Expected: line 8, column 5");
    println!(
        "Actual: line {}, column {}",
        key.span().start().unwrap().line(),
        key.span().start().unwrap().column()
    );
    assert_eq!(key.span().start().unwrap().line(), 8);
    assert_eq!(key.span().start().unwrap().column(), 5);
    println!(
        "{}",
        visualize_position(
            yaml,
            key.span().start().unwrap().line(),
            key.span().start().unwrap().column()
        )
    );

    // Get the third item from the sequence
    let third_item = sequence.get_mapping(2).unwrap();

    // Third item should start at the key name, not the dash
    assert!(third_item.span().start().is_some());
    let third_item_start = third_item.span().start().unwrap();
    println!("\n=== Third Item Start ===");
    println!("Expected: line 10, column 7"); // This points to the 'n' in "name"
    println!(
        "Actual: line {}, column {}",
        third_item_start.line(),
        third_item_start.column()
    );
    assert_eq!(third_item_start.line(), 10);
    assert_eq!(third_item_start.column(), 7);
    println!(
        "{}",
        visualize_position(yaml, third_item_start.line(), third_item_start.column())
    );

    // Check the list in the third item
    let inner_list = third_item.get_sequence("list").unwrap();
    println!("\n=== Inner List Start ===");
    println!("Expected: line 13, column 5");
    println!(
        "Actual: line {}, column {}",
        inner_list.span().start().unwrap().line(),
        inner_list.span().start().unwrap().column()
    );
    assert_eq!(inner_list.span().start().unwrap().line(), 13);
    assert_eq!(inner_list.span().start().unwrap().column(), 5);
    println!(
        "{}",
        visualize_position(
            yaml,
            inner_list.span().start().unwrap().line(),
            inner_list.span().start().unwrap().column()
        )
    );

    // Check the list items exist and have correct content
    assert_eq!(inner_list.len(), 2);
    let list_item_1 = inner_list.get_scalar(0).unwrap();
    assert_eq!(list_item_1.as_str(), "list item 1");
    println!("\n=== 'list item 1' Start ===");
    println!("Expected: line 13, column 7");
    println!(
        "Actual: line {}, column {}",
        list_item_1.span().start().unwrap().line(),
        list_item_1.span().start().unwrap().column()
    );
    assert_eq!(list_item_1.span().start().unwrap().line(), 13);
    assert_eq!(list_item_1.span().start().unwrap().column(), 7);
    println!(
        "{}",
        visualize_position(
            yaml,
            list_item_1.span().start().unwrap().line(),
            list_item_1.span().start().unwrap().column()
        )
    );

    let list_item_2 = inner_list.get_scalar(1).unwrap();
    assert_eq!(list_item_2.as_str(), "list item 2");
    println!("\n=== 'list item 2' Start ===");
    println!("Expected: line 14, column 7");
    println!(
        "Actual: line {}, column {}",
        list_item_2.span().start().unwrap().line(),
        list_item_2.span().start().unwrap().column()
    );
    assert_eq!(list_item_2.span().start().unwrap().line(), 14);
    assert_eq!(list_item_2.span().start().unwrap().column(), 7);
    println!(
        "{}",
        visualize_position(
            yaml,
            list_item_2.span().start().unwrap().line(),
            list_item_2.span().start().unwrap().column()
        )
    );
}

#[test]
fn verify_span_existence() {
    // Create a YAML document with clear structure
    let yaml = r#"---
mappings:
  key1: value1
  key2: value2
  
sequences:
  - item1
  - item2
  
nested:
  outer:
    inner: value
"#;

    println!("\n=== YAML Document ===\n{}", yaml);

    // Parse the YAML document
    let node = parse_yaml(0, yaml).unwrap();
    let root = node.as_mapping().unwrap();

    // The root mapping should have valid start and end markers
    assert!(root.span().start().is_some());
    assert!(root.span().end().is_some());

    // Root mapping info
    println!("\n=== Root Mapping Start ===");
    println!("Expected: line 2, column 1");
    println!(
        "Actual: line {}, column {}",
        root.span().start().unwrap().line(),
        root.span().start().unwrap().column()
    );
    assert_eq!(root.span().start().unwrap().line(), 2);
    assert_eq!(root.span().start().unwrap().column(), 1);
    println!(
        "{}",
        visualize_position(
            yaml,
            root.span().start().unwrap().line(),
            root.span().start().unwrap().column()
        )
    );

    // Check the "mappings" section exists
    let mappings = root.get_mapping("mappings").unwrap();
    println!("\n=== 'mappings' Section Start ===");
    println!("Expected: line 3, column 3");
    println!(
        "Actual: line {}, column {}",
        mappings.span().start().unwrap().line(),
        mappings.span().start().unwrap().column()
    );
    assert_eq!(mappings.span().start().unwrap().line(), 3);
    assert_eq!(mappings.span().start().unwrap().column(), 3);
    println!(
        "{}",
        visualize_position(
            yaml,
            mappings.span().start().unwrap().line(),
            mappings.span().start().unwrap().column()
        )
    );

    // Check keys exist in mappings
    assert!(mappings.contains_key("key1"));
    assert!(mappings.contains_key("key2"));

    // Get key1 and check position
    let key1 = mappings.keys().find(|k| k.as_str() == "key1").unwrap();
    println!("\n=== 'key1' Key Start ===");
    println!("Expected: line 3, column 3");
    println!(
        "Actual: line {}, column {}",
        key1.span().start().unwrap().line(),
        key1.span().start().unwrap().column()
    );
    assert_eq!(key1.span().start().unwrap().line(), 3);
    assert_eq!(key1.span().start().unwrap().column(), 3);
    println!(
        "{}",
        visualize_position(
            yaml,
            key1.span().start().unwrap().line(),
            key1.span().start().unwrap().column()
        )
    );

    // Get value1 and check content and position
    let value1 = mappings.get_scalar("key1").unwrap();
    assert_eq!(value1.as_str(), "value1");
    println!("\n=== 'value1' Scalar Start ===");
    println!("Expected: line 3, column 9");
    println!(
        "Actual: line {}, column {}",
        value1.span().start().unwrap().line(),
        value1.span().start().unwrap().column()
    );
    assert_eq!(value1.span().start().unwrap().line(), 3);
    assert_eq!(value1.span().start().unwrap().column(), 9);
    println!(
        "{}",
        visualize_position(
            yaml,
            value1.span().start().unwrap().line(),
            value1.span().start().unwrap().column()
        )
    );

    // Check the "sequences" section exists
    let sequences = root.get_sequence("sequences").unwrap();
    println!("\n=== 'sequences' Section Start ===");
    println!("Expected: line 7, column 3");
    println!(
        "Actual: line {}, column {}",
        sequences.span().start().unwrap().line(),
        sequences.span().start().unwrap().column()
    );
    assert_eq!(sequences.span().start().unwrap().line(), 7);
    assert_eq!(sequences.span().start().unwrap().column(), 3);
    println!(
        "{}",
        visualize_position(
            yaml,
            sequences.span().start().unwrap().line(),
            sequences.span().start().unwrap().column()
        )
    );

    // Check sequence has the right items by content
    assert_eq!(sequences.len(), 2);
    let item1 = sequences.get_scalar(0).unwrap();
    assert_eq!(item1.as_str(), "item1");
    println!("\n=== 'item1' Scalar Start ===");
    println!("Expected: line 7, column 5");
    println!(
        "Actual: line {}, column {}",
        item1.span().start().unwrap().line(),
        item1.span().start().unwrap().column()
    );
    assert_eq!(item1.span().start().unwrap().line(), 7);
    assert_eq!(item1.span().start().unwrap().column(), 5);
    println!(
        "{}",
        visualize_position(
            yaml,
            item1.span().start().unwrap().line(),
            item1.span().start().unwrap().column()
        )
    );

    // Check nested section exists
    let nested = root.get_mapping("nested").unwrap();
    println!("\n=== 'nested' Section Start ===");
    println!("Expected: line 11, column 3");
    println!(
        "Actual: line {}, column {}",
        nested.span().start().unwrap().line(),
        nested.span().start().unwrap().column()
    );
    assert_eq!(nested.span().start().unwrap().line(), 11);
    assert_eq!(nested.span().start().unwrap().column(), 3);
    println!(
        "{}",
        visualize_position(
            yaml,
            nested.span().start().unwrap().line(),
            nested.span().start().unwrap().column()
        )
    );

    // Check outer exists within nested
    let outer = nested.get_mapping("outer").unwrap();
    println!("\n=== 'outer' Section Start ===");
    println!("Expected: line 12, column 5");
    println!(
        "Actual: line {}, column {}",
        outer.span().start().unwrap().line(),
        outer.span().start().unwrap().column()
    );
    assert_eq!(outer.span().start().unwrap().line(), 12);
    assert_eq!(outer.span().start().unwrap().column(), 5);
    println!(
        "{}",
        visualize_position(
            yaml,
            outer.span().start().unwrap().line(),
            outer.span().start().unwrap().column()
        )
    );

    // Check inner value is correct
    let inner = outer.get_scalar("inner").unwrap();
    assert_eq!(inner.as_str(), "value");
    println!("\n=== 'inner' Value Start ===");
    println!("Expected: line 12, column 12");
    println!(
        "Actual: line {}, column {}",
        inner.span().start().unwrap().line(),
        inner.span().start().unwrap().column()
    );
    assert_eq!(inner.span().start().unwrap().line(), 12);
    assert_eq!(inner.span().start().unwrap().column(), 12);
    println!(
        "{}",
        visualize_position(
            yaml,
            inner.span().start().unwrap().line(),
            inner.span().start().unwrap().column()
        )
    );
}

#[test]
fn complex_document_structure() {
    // A more complex document with deep nesting and various YAML structures
    let yaml = r#"---
apiVersion: v1
kind: ConfigMap
metadata:
  name: complex-config
  namespace: test
data:
  config.yaml: |
    server:
      host: example.com
      port: 8080
      timeouts:
        connect: 5s
        read: 10s
        write: 5s
      features:
        - name: feature1
          enabled: true
          settings:
            retries: 3
            backoff: exponential
        - name: feature2
          enabled: false
          settings:
            retries: 0
        - name: feature3
          enabled: true
          dependencies:
            - feature1
            - name: external1
              url: https://api.example.com
  secrets:
    - user: admin
      password: password123
    - user: reader
      password: readonly456
      permissions:
        - read
        - list
"#;

    println!("\n=== YAML Document (showing first few lines) ===");
    let first_lines: Vec<&str> = yaml.lines().take(10).collect();
    for line in first_lines {
        println!("{}", line);
    }
    println!("... (truncated for brevity)");

    // Parse the document
    let node = parse_yaml(0, yaml).unwrap();
    let root = node.as_mapping().unwrap();

    // Verify root has a valid span
    assert!(root.span().start().is_some());

    // Root mapping info
    println!("\n=== Root Mapping Start ===");
    println!("Expected: line 2, column 1");
    println!(
        "Actual: line {}, column {}",
        root.span().start().unwrap().line(),
        root.span().start().unwrap().column()
    );
    assert_eq!(root.span().start().unwrap().line(), 2);
    assert_eq!(root.span().start().unwrap().column(), 1);
    println!(
        "{}",
        visualize_position(
            yaml,
            root.span().start().unwrap().line(),
            root.span().start().unwrap().column()
        )
    );

    // Check top level fields exist and have correct content
    assert!(root.contains_key("apiVersion"));
    assert!(root.contains_key("kind"));
    assert!(root.contains_key("metadata"));
    assert!(root.contains_key("data"));

    // apiVersion value
    let api_version = root.get_scalar("apiVersion").unwrap();
    assert_eq!(api_version.as_str(), "v1");
    println!("\n=== 'apiVersion' Value Start ===");
    println!("Expected: line 2, column 13");
    println!(
        "Actual: line {}, column {}",
        api_version.span().start().unwrap().line(),
        api_version.span().start().unwrap().column()
    );
    assert_eq!(api_version.span().start().unwrap().line(), 2);
    assert_eq!(api_version.span().start().unwrap().column(), 13);
    println!(
        "{}",
        visualize_position(
            yaml,
            api_version.span().start().unwrap().line(),
            api_version.span().start().unwrap().column()
        )
    );

    // kind value
    let kind = root.get_scalar("kind").unwrap();
    assert_eq!(kind.as_str(), "ConfigMap");
    println!("\n=== 'kind' Value Start ===");
    println!("Expected: line 3, column 7");
    println!(
        "Actual: line {}, column {}",
        kind.span().start().unwrap().line(),
        kind.span().start().unwrap().column()
    );
    assert_eq!(kind.span().start().unwrap().line(), 3);
    assert_eq!(kind.span().start().unwrap().column(), 7);
    println!(
        "{}",
        visualize_position(
            yaml,
            kind.span().start().unwrap().line(),
            kind.span().start().unwrap().column()
        )
    );

    // metadata section
    let metadata = root.get_mapping("metadata").unwrap();
    println!("\n=== 'metadata' Section Start ===");
    println!("Expected: line 5, column 3");
    println!(
        "Actual: line {}, column {}",
        metadata.span().start().unwrap().line(),
        metadata.span().start().unwrap().column()
    );
    assert_eq!(metadata.span().start().unwrap().line(), 5);
    assert_eq!(metadata.span().start().unwrap().column(), 3);
    println!(
        "{}",
        visualize_position(
            yaml,
            metadata.span().start().unwrap().line(),
            metadata.span().start().unwrap().column()
        )
    );

    // Get name field
    let name = metadata.get_scalar("name").unwrap();
    assert_eq!(name.as_str(), "complex-config");
    println!("\n=== 'name' Field Value Start ===");
    println!("Expected: line 5, column 9");
    println!(
        "Actual: line {}, column {}",
        name.span().start().unwrap().line(),
        name.span().start().unwrap().column()
    );
    assert_eq!(name.span().start().unwrap().line(), 5);
    assert_eq!(name.span().start().unwrap().column(), 9);
    println!(
        "{}",
        visualize_position(
            yaml,
            name.span().start().unwrap().line(),
            name.span().start().unwrap().column()
        )
    );

    // data section
    let data = root.get_mapping("data").unwrap();
    println!("\n=== 'data' Section Start ===");
    println!("Expected: line 8, column 3");
    println!(
        "Actual: line {}, column {}",
        data.span().start().unwrap().line(),
        data.span().start().unwrap().column()
    );
    assert_eq!(data.span().start().unwrap().line(), 8);
    assert_eq!(data.span().start().unwrap().column(), 3);
    println!(
        "{}",
        visualize_position(
            yaml,
            data.span().start().unwrap().line(),
            data.span().start().unwrap().column()
        )
    );

    // config.yaml field
    let config_yaml = data.get_scalar("config.yaml").unwrap();
    println!("\n=== 'config.yaml' Field Start ===");
    println!("Expected: line 9, column 5");
    println!(
        "Actual: line {}, column {}",
        config_yaml.span().start().unwrap().line(),
        config_yaml.span().start().unwrap().column()
    );
    assert_eq!(config_yaml.span().start().unwrap().line(), 9);
    assert_eq!(config_yaml.span().start().unwrap().column(), 5);
    println!(
        "{}",
        visualize_position(
            yaml,
            config_yaml.span().start().unwrap().line(),
            config_yaml.span().start().unwrap().column()
        )
    );

    // secrets section
    let secrets = data.get_sequence("secrets").unwrap();
    assert_eq!(secrets.len(), 2);
    println!("\n=== 'secrets' Section Start ===");
    println!("Expected: line 33, column 5");
    println!(
        "Actual: line {}, column {}",
        secrets.span().start().unwrap().line(),
        secrets.span().start().unwrap().column()
    );
    assert_eq!(secrets.span().start().unwrap().line(), 33);
    assert_eq!(secrets.span().start().unwrap().column(), 5);
    println!(
        "{}",
        visualize_position(
            yaml,
            secrets.span().start().unwrap().line(),
            secrets.span().start().unwrap().column()
        )
    );
}
