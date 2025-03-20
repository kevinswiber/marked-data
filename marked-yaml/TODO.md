# TODO: Fix YAML Mapping Style Detection

## Problem

The `yaml-rust2` library doesn't provide information about whether a mapping is flow-style (using braces `{}`) or block-style in its parser events. This limitation affects source position accuracy in Marked YAML, causing tests to fail and potentially causing issues for applications that rely on precise source positions.

## Affected Functionality

- Flow-style mapping spans don't accurately point to the opening brace `{`
- Mappings in sequences have positions pointing to sequence markers rather than the mapping start

## Failed Tests

1. `sequence_of_mappings_spans` - Expects mappings in sequences to start at column 7 (where the first key begins) but gets column 3 (where the dash is)
2. `flow_style_mapping_spans` - Expects flow-style mappings to start at the `{` character, but parser doesn't provide this information
3. `complex_document_structure` - Expects the 'metadata' section to start at column 1, but gets column 3 based on first key position

## Possible Solutions

### Option 1: Fork yaml-rust2

Create a fork of the `yaml-rust2` crate and enhance the parser to include style information:

1. Modify the `Event::MappingStart` enum variant to include a style parameter (similar to how `Event::Scalar` includes `TScalarStyle`)
2. Add a `TMappingStyle` enum in the scanner similar to `TScalarStyle`
3. Track mapping style during parsing and include it in the `MappingStart` event
4. Update the `MarkedLoader` to use this information

**Pros:** Clean, complete solution that would provide accurate information without hacks
**Cons:** Maintaining a fork, upgrading when upstream changes

### Option 2: Enhance MarkedLoader

Enhance the `MarkedLoader` to track document text and positions:

1. Store the original document text in the `MarkedLoader`
2. For each mark, look at the character at that position to determine if it's a flow-style mapping
3. Use this information to adjust mapping spans

**Pros:** No need to fork a dependency
**Cons:** More complex code, potential edge cases

### Option 3: Implement Better Heuristics

Implement better position-based heuristics:

1. Detect mapping style based on indentation patterns
2. Analyze first key positions to infer mapping style
3. Use `yaml-rust` parsing events sequence to distinguish styles

**Pros:** No dependency changes
**Cons:** Still fragile, likely to break with complex YAML

## Recommended Approach

Option 1 (forking yaml-rust2) provides the cleanest solution. Steps would include:

1. Fork the `yaml-rust2` repository
2. Add mapping style information to the parser
3. Update our dependency to use the forked version
4. Modify `MarkedLoader` to utilize the new style information
5. Consider contributing changes back to the upstream repository

## Timeline

- Near-term: Document limitations in tests and code
- Mid-term: Implement one of the solutions above
- Long-term: Work with yaml-rust2 maintainers to incorporate changes upstream if possible

## References

- [yaml-rust2 crate](https://crates.io/crates/yaml-rust2)
- [YAML spec](https://yaml.org/spec/1.2.2/)
- Failed test details in `tests/loader_tests.rs` 