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

## Flow-Style Mapping Detection

The `yaml-rust2` library does not provide information about whether a mapping is flow-style (using braces `{}`) or block-style in its parser events. This limitation affects source position accuracy for mappings in Marked YAML.

For example, in YAML like:

```yaml
root: { key1: value1, key2: value2 }
```

The parser does not indicate that the mapping is flow-style, and the position information does not accurately point to the opening brace `{`. This affects how span information is reported for flow-style mappings.

Currently, some tests that validate position accuracy for flow-style mappings and mappings within sequences may fail due to this limitation.

## Potential Solutions

To properly fix these limitations, one of the following approaches would be needed:

1. Fork `yaml-rust2` and enhance the `Event::MappingStart` to include style information (similar to how scalar style is included in `Event::Scalar`)

2. Enhance the `MarkedLoader` to track the document text alongside parsing to examine characters at specific positions

## Future Work

We plan to address these limitations in a future release. If you find cases where span information is inaccurate, please report them as issues in the repository.

For now, applications should be aware that:
- Flow-style mapping spans may not precisely point to the opening brace
- Mappings in sequences may have position information that points to sequence markers rather than the mapping start
