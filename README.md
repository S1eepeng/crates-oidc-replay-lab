# s1eepeng-crates-oidc-replay-lab

A small, functional Rust calculator used to study crates.io Trusted Publishing,
GitHub Actions OIDC token exchange, and replay behavior.

```rust
use s1eepeng_crates_oidc_replay_lab::{Operation, calculate};

assert_eq!(calculate(2.0, Operation::Multiply, 3.0).unwrap(), 6.0);
```

The package also installs a command-line program:

```console
oidc-calc 10 div 4
2.5
```

This is a security research package. It is not intended for production use.

