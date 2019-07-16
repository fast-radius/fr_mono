# FR Mono

This is a library that takes a string and turns it into a 3D triangle mesh.
You can then write this 3D mesh as an STL.

To use this library, call this function:

```rust
pub fn to_stl(s: &str) -> Vec<Triangle>
```

The current character set is `0-9`, `A-Z`, and `-`.
