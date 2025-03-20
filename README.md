# Marked data

This is a collection of libraries which offer data provenance facilities and
then use that to provide a variety of marked data structures.

The data marks are intended to be similar to, though not entirely the same as
data structures seen in other systems as `Span` or similar.  Instead data marks
are a combination of that span-like data and also file sources.  The intent here
being that combining data from multiple sources is a safe and sensible thing to
do and that after doing so, it is important to be able to discover where something
came from.

Currently we provide:

* `marked-yaml`: A deliberately simplified YAML subset which supports data marking

## Marked YAML

The `marked-yaml` library offers a subset of YAML designed for configuration files
and similar use.  The data it reads in is "marked" with its origin.  If you want
to use `marked-yaml` with your existing `serde` applications, you can enable the
`serde` feature, and if you want the errors produced by the `marked-yaml`
deserializer to include nice paths to any problem, along with ensuring the marker
for the problem area is populated in any errors, use the `serde-path` feature.

### Fork Enhancements (`bangarang` branch)

This fork enhances the original marked-data project with improved position tracking and flow mapping support in the `marked-yaml` library:

1. **Enhanced Flow Mapping Position Tracking**
   - Added precise tracking of flow mapping opening and closing brace positions
   - Improved position accuracy for flow-style YAML mappings (using `{}` syntax)
   - Added support for nested flow mappings with accurate position information

2. **Test Suite Improvements**
   - Added comprehensive test suite for flow-style mapping position tracking
   - Added visualization helpers for position debugging
   - Added tests for complex document structures with mixed block and flow styles

3. **Developer Notes**
   - Position tracking is now more accurate for flow-style mappings
   - Simple adjustments are made for flow mapping opening braces (adjusting position by -2 columns)
   - Some complex cases may still have limitations pending further parser improvements

These enhancements are particularly useful for:
- IDEs and editors that need precise position information for code navigation
- Tools that provide error messages or diagnostics for YAML documents
- Applications that need to map YAML structures back to their source positions

Note: While most position tracking works well, some complex cases with flow-style mappings may still have limitations that will be addressed in future updates.
