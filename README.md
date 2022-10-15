## radix-sort

Implementation of the digital sorting algorithm (LSD) for positive integers.

### Usage example:
```rust
use radix_sort::lsd_sort;

fn main(){
    let mut arr: Vec<u32> = vec![4, 678, 234, 1, 8904, 45, 909, 13, 45333, 23000];
    lsd_sort(&mut arr);
    assert_eq!(arr, vec![1, 4, 13, 45, 234, 678, 909, 8904, 23000, 45333]);
}
```

### Cargo.toml
```bash
[dependencies]
radix-trie = {git = "https://github.com/mingendo/radix-sort.git", branch="main"}
```

