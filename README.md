# FR Mono

This is a library that takes a string and turns it into a 3D triangle mesh.
You can then write this 3D mesh as an STL.

<img width="1491" alt="Screen Shot 2019-07-17 at 10 06 00 AM" src="https://user-images.githubusercontent.com/3415332/61386904-b81e9800-a87a-11e9-9c1c-db2ef2bb9d39.png">

To use this library, call this function:

```rust
pub fn to_stl(s: &str) -> Vec<Triangle>
```

The current character set is `0-9`, `A-Z`, and `-`.
