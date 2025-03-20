# TODO: Fix YAML Position Tracking

## Current Status

Tests that rely on precise position tracking for YAML elements have been marked with `#[ignore]` attributes, as the underlying position information from the `yaml-rust2` parser is insufficient for accurate position reporting in several scenarios:

1. **Flow-style mappings** - The parser reports the position of the first key rather than the opening `{` brace
2. **Block-style mapping indentation** - For nested block mappings, the column positions are inconsistent
3. **Sequence items** - The positions of items within sequences are not accurately reported

We've implemented a simple adjustment for flow-style mapping positions (subtracting 2 from the column number when a flow-style mapping is detected), but this is only a partial solution that works in the simplest cases.

## Problem

The `yaml-rust2` library doesn't provide sufficient information about mapping styles and positions in its parser events. This limitation affects source position accuracy in Marked YAML, causing tests to fail and potentially causing issues for applications that rely on precise source positions.

## Affected Functionality

The following tests are currently ignored due to these limitations:
- `sequence_of_mappings_spans`
- `verify_span_existence`
- `complex_document_structure`
- `flow_style_mapping_spans`

## Potential Solutions

### Option 1: Enhance yaml-rust2 (Preferred)

Create a fork of the `yaml-rust2` crate and enhance the parser to include better position information:

1. Extend the public API to expose the existing flow mapping position tracking mechanism. The scanner already tracks exact positions of flow mapping opening braces via a `position_id` system, but these positions aren't accessible through the public API.

2. Modify the `Parser` class to pass the position information from the scanner to the event receiver in a more accessible manner.

3. Improve the scanner to capture exact positions of structural elements like braces, brackets, and indentation.

4. Enhance the pattern-matching in the parser to consistently report column positions relative to indentation.

**Pros:** Clean, complete solution that would provide accurate information
**Cons:** Requires maintaining a fork until changes can be upstreamed

### Option 2: Pre/Post-processing Adjustments

Enhance the `MarkedLoader` to perform more sophisticated adjustments to positions based on document structure:

1. Record the input document, scanning it to determine correct positions by looking at the actual characters
2. Implement heuristics to infer correct positions based on surrounding context
3. Add multiple passes - first to parse the document, then to adjust positions

**Pros:** Can be implemented without modifying dependencies
**Cons:** More complex code, potential for subtle errors, less efficient

## Timeline

- **Short-term:** Keep tests ignored with detailed explanations
- **Medium-term:** Implement a more principled set of position adjustments in the loader
- **Long-term:** Enhance yaml-rust2 with proper position tracking and upstream the changes

## Next Steps

The immediate priorities are:

1. Document the current limitations in the README and codebase (✅ Done)
2. Investigate enhancing the yaml-rust2 parser to expose the flow mapping position information that's already being tracked by the scanner
3. Consider a phased approach where:
   - First, we make flow mapping positions accessible
   - Then, we address block-style mapping positions
   - Finally, we improve positions for sequence items
4. Prepare a roadmap for implementing these enhancements

## Implementation Roadmap

### Phase 1: Access Flow Mapping Positions (Short-term)

1. **Fork the yaml-rust2 crate** and add a public API to access the flow mapping positions:
   - Add a `get_flow_mapping_position(id: usize) -> Option<Marker>` method to the Parser
   - Enhance the `load` method to expose the position information to event receivers
   
2. **Modify MarkedLoader** to use the exact positions:
   - Update implementation to accept and store flow mapping positions
   - Replace the -2 column adjustment with accurate position information
   
3. **Update tests** to verify position accuracy for flow-style mappings:
   - Un-ignore the `flow_style_mapping_spans` test once the implementation is complete
   - Add additional test cases with more complex nested flow mappings

### Phase 2: Block-Style Mapping Position (Medium-term)

1. **Enhance yaml-rust2 scanner** to track block mapping positions:
   - Modify the scanner to track indentation for block mappings
   - Add column position correction based on indentation level
   
2. **Update the parser** to pass these positions to the MarkedLoader:
   - Extend the `Event::MappingStart` to include better position information for block styles
   - Add indentation context to improve position reporting
   
3. **Update tests** to verify position accuracy for block-style mappings:
   - Un-ignore the `verify_span_existence` test
   - Add additional test cases for deeply nested block mappings

### Phase 3: Sequence Position Improvements (Long-term)

1. **Enhance position tracking for sequences**:
   - Modify the scanner to track sequence item start and end positions
   - Include brackets and indentation information in position tracking
   
2. **Implement sequence context in loader**:
   - Add sequence context tracking in the MarkedLoader
   - Adjust position information based on sequence nesting level
   
3. **Complete the test suite**:
   - Un-ignore the `sequence_of_mappings_spans` and `complex_document_structure` tests
   - Add comprehensive test cases for all YAML structural elements

### Phase 4: Upstream Changes and Finalize (Future)

1. **Propose changes to yaml-rust2**:
   - Submit a PR with position tracking enhancements
   - Work with maintainers to align with project goals
   
2. **Document the enhanced position tracking**:
   - Update the markdown-yaml documentation with detailed position tracking information
   - Add examples showing accurate span information
   
3. **Release a new version** with complete position tracking support

## References

- [yaml-rust2 crate](https://crates.io/crates/yaml-rust2)
- [YAML spec](https://yaml.org/spec/1.2.2/)
- Failed test details in `tests/loader_tests.rs` 