Now I understand the issue. After reviewing the code, here's what's happening:
In loader.rs, when handling YAML syntax, the parser creates markers for different events in the YAML parsing process.
For mapping nodes, the issue is in how the spans are created:
When a MappingStart event occurs, it saves the marker position (MappingWaitingOnKey(mark, MappingHash::new())) at line 224-225.
The problem is this marker comes from the YAML parser and points to the colon after the key, not the first character of the key itself.
When a MappingEnd event occurs, it creates a span using that starting marker and creates a MarkedMappingNode with it (lines 236-237).
For flow-style mappings (with braces), the marker handling is different:
The MappingStart event for flow-style mappings has its marker at the opening brace {
The MappingEnd event for flow-style mappings has its marker at the closing brace }
This is why the flow-style mapping tests pass - they're already getting the correct positions.
For block-style mappings (without braces), we need to fix the start position:
Currently, the start position is at the colon (:) character between the key and value.
We need to change it to be at the first character of the key.
Scalar nodes (like mapping keys) are created in the Event::Scalar branch:
Scalar nodes record their position correctly at their first character (line 350).
However, when a scalar is a mapping key, that position information isn't used for the mapping's start position.
The fix would need to modify how the start position for mappings is determined in block-style YAML. Instead of using the marker from the MappingStart event (which points to the colon), it should use the position of the first key's first character.