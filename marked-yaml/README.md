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

The `yaml-rust2` library has several limitations in its position tracking, especially for certain YAML structures:

1. **Flow-Style Mapping Positions**: The parser doesn't correctly report the position of opening/closing braces (`{}`) for flow-style mappings. Instead, it reports the position of the first key.

   While the scanner tracks the exact position of opening braces for flow-style mappings via a `position_id` mechanism, these positions are not fully accessible through the public API.

2. **Block-Style Mapping Indentation**: For nested block mappings, position information may not accurately reflect the correct indentation level.

3. **Sequence Items**: The positions of items within sequences (especially mappings within sequences) may not be accurately reported.

These limitations affect accuracy of span information in the marked-yaml output. We've implemented a simple adjustment for flow-style mapping positions by subtracting two columns from the reported position when detecting a flow-style mapping, but this is a partial solution that works only in simple cases.

### Current Workarounds 

We currently:
- Apply a simple position adjustment for flow-style mappings (subtracting 2 from column position)
- Document known limitations in the codebase
- Ignore tests that validate exact position information until a more comprehensive solution is implemented

### Long-term Solutions

To properly fix these limitations, one of the following approaches would be needed:

1. Enhance `yaml-rust2` to provide more accurate position information in parser events, particularly for flow-style mappings and indentation-sensitive structures. This could involve extending the public API to expose the existing flow mapping position tracking.

2. Implement more sophisticated position tracking in `marked-yaml` that analyzes the YAML document in a post-processing step.

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

We have developed a detailed, phased implementation plan to address the position tracking limitations:

1. **Phase 1 (Short-term)**: Access flow mapping positions by enhancing yaml-rust2
2. **Phase 2 (Medium-term)**: Improve block-style mapping position tracking
3. **Phase 3 (Long-term)**: Enhance sequence position information
4. **Phase 4 (Future)**: Upstream changes and finalize

See the [Implementation Roadmap](TODO.md#implementation-roadmap) for more details on each phase and specific tasks to be completed.
