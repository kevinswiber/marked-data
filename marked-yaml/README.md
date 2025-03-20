# Marked YAML

This library builds atop [`yaml-rust2`][yaml-rust2] to provide a YAML AST which
includes the marks for where the YAML data comes from. It explicitly operates
at a low level, providing only the _base_ **safe** YAML types (i.e. the vanilla
tags `tag:yaml.org,2002:seq`, `tag:yaml.org,2002:map`, and `tag:yaml.org,2002:str`)

[yaml-rust2]: https://crates.io/crates/yaml-rust2

The subset of YAML which is supported is quite deliberately limited in order
that users of this crate will implicitly discourage complex use of YAML which
is harder to manage user expectations with. As an example, the mapping type
in this crate explicitly only permits scalars as keys, and since all scalars
are treated as strings, mappings always have string keys.

The primary value of this kind of representation of YAML data is to allow
applications which want to be very explicit about where input came from an
opportunity to do this in a way which normal YAML parsers do not allow.

# Using Marked YAML

Currently this library only supports loading YAML from strings,
but this is sufficient for most users' purposes. We would not
recommend an un-streamed processing engine for massive data anyway.

To load some YAML you simply need to:

```rust
let node = marked_yaml::parse_yaml(0, r#"
toplevel: must be a mapping
but:
 - it
 - may
 - contain lists
and:
 mappings: are permitted
 as: sub-mappings
"#);
assert!(node.is_ok());
```

Parsing a valid YAML file may fail because `marked_yaml` adds some
additional constraints:

- The top level of the YAML **MUST** be one of a mapping or a sequence. This is
  controlled by the loader options.
- Mapping keys **MUST** be scalars (strings).
- Aliases and anchors **MAY NOT** be used (though this limit may be lifted in the future).

In addition, you can convert between `marked_yaml::Node` and `yaml_rust::Yaml`
though doing so will not give you any useful markers.

# Known Limitations

## Position Tracking Accuracy

The `marked-yaml` library now provides accurate position tracking for most YAML structures. Previously, there were limitations in the position tracking due to implementation details in the `yaml-rust2` library, but these issues have been resolved:

1. **Flow-Style Mapping Positions**: The library now correctly reports the position of opening/closing braces (`{}`) for flow-style mappings, pointing to the exact position of the opening brace.

2. **Block-Style Mapping Indentation**: Block mappings now report their position as the position of their first key, which matches standard YAML parsing expectations.

3. **Sequence Items**: The positions of items within sequences are now accurately reported, including mappings within sequences.

These improvements enhance the accuracy of span information in the marked-yaml output, making it more useful for applications that rely on precise source positions, such as error reporting, document validation, or source-to-source transformations.

### Implementation Details

We've implemented several enhancements to make position tracking accurate:

1. For flow-style mappings, we use the position ID mechanism provided by the scanner to get the exact position of opening braces.

2. For block-style mappings, we adjust the reported position to point to the first key in the mapping.

3. For sequence items, we properly track the position of each item based on its type and context.

### Future Enhancements

While all tests are now passing, we continue to look for ways to improve position tracking. Some areas for future enhancement include:

1. Upstream changes to `yaml-rust2` to make position tracking more robust at the parser level.

2. More sophisticated handling of edge cases in complex nested structures.

3. Additional test cases for complex YAML documents with varied styles and nesting levels.

## Other Limitations

- Aliases and anchors **MAY NOT** be used (though this limitation may be lifted in the future).
- Mapping keys **MUST** be scalars (strings).
- The top level of the YAML **MUST** be one of a mapping or a sequence (controlled by loader options).

## Future Work

We plan to address these limitations in a future release. If you find cases where span information is inaccurate, please report them as issues in the repository.

For now, applications should be aware that:
- Flow-style mapping spans may not precisely point to the opening brace
- Mappings in sequences may have position information that points to sequence markers rather than the mapping start

## Implementation Plan

We've successfully implemented our roadmap for improving position tracking:

## ✅ Phase 1: Flow-Style Mapping Position Tracking
- The marked-yaml library now correctly reports positions for flow-style mappings
- Flow mapping opening braces are precisely tracked
- All related tests are now passing

## ✅ Phase 2: Block-Style Mapping Position Tracking
- Block mappings now have their positions correctly reported as the position of their first key
- Nested block mappings maintain proper position information
- All related tests are now passing

## ✅ Phase 3: Sequence Position Improvements
- Sequence items now have their positions correctly tracked
- Mappings within sequences maintain proper position information
- All related tests are now passing

## ✅ Phase 4: Complex Document Structure Support
- Complex nested YAML structures now have accurate position tracking
- All tests are now passing, including the previously ignored tests

See the [Implementation Status](TODO.md) document for more details on the completed work and future plans.
