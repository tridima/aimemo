# aimemo

**Modified for aimemo, 2026-10-01:** this README replaces the upstream README.

aimemo is an unofficial fork of [Memvid](https://github.com/memvid/memvid),
a single-file Rust memory library, with local storage quotas removed.
It preserves the `memvid-core` 2.0.140 crate name, `memvid_core` imports, and
`.mv2` format. [Upstream base](https://github.com/memvid/memvid/commit/e6bd9f7b9c38cd8d5370fa0fc936ac1dcd751813).

## Changes

- `capacity_limit()` returns `u64::MAX`: the 50 MiB free-tier cap and capacity
  values in existing tickets no longer limit local writes.
- `get_capacity()` and `stats().capacity_bytes` report `u64::MAX`; remaining
  capacity and utilization use that value, not available disk space.
- Regression tests cover files above 50 MiB, reopening and appending, and
  persisted ticket capacities. The quota-rejection test is replaced.
- The standalone Cargo workspace builds inside another repository; source
  packages include the license, maintenance instructions, and regression tests.
- CI is manual-only. The inherited Docker publishing workflow targeting the
  upstream Memvid CLI image is removed.

Ticket metadata, tier definitions, signature validation, integrity checks,
write-ahead logging, and file-format limits remain intact. Disk and system
resources still bound storage. Upstream clients may enforce retained ticket
capacities. Hosted services are outside this fork.

## Use this source

Adjust the path relative to the consuming `Cargo.toml`:

```toml
[dependencies]
memvid-core = { path = "../third_party/aimemo", default-features = false, features = ["lex"] }
```

Or use the fork through Git:

```toml
[dependencies]
memvid-core = { git = "https://github.com/tridima/aimemo.git", branch = "main", default-features = false, features = ["lex"] }
```

Commit the application's `Cargo.lock` to pin the resolved Git commit; use
`--locked` for reproducible builds. For an explicit source pin, replace `branch`
with `rev` containing the chosen full commit SHA. Preserve required application
features; these examples select lexical search only. Confirm the resolved source
with `cargo tree -i memvid-core` in the application.

This is Rust source, not a replacement published SDK or CLI. Python, Node, and
CLI native components must be rebuilt against this fork. Upstream package
installs do not include the change. See [upstream docs](https://docs.memvid.com)
for the broader API and [AGENTS.md](AGENTS.md) for maintenance instructions.

## Build and verify

Rust 1.90 is required and pinned by `rust-toolchain.toml`. From this checkout:

```sh
cargo build --locked --release --no-default-features --features lex
cargo test --locked --no-default-features --features lex --test local_capacity
cargo test --locked --no-default-features --features lex --lib persisted_capacity_limit_ignored_for_local_storage
cargo clippy --locked --no-default-features --features lex --lib --test local_capacity
cargo fmt --all -- --check
```

The capacity regression checks data after reopening a file larger than 50 MiB.
Verify the same behavior through the consuming application after integration.
The retained upstream lockfile contains three yanked dependency versions; Cargo
reports them during packaging, while locked builds and package verification pass.

## License and attribution

[Apache-2.0](LICENSE) Section 2 permits derivative works, including this local
capacity change. Redistribution remains subject to Section 4: include the license,
mark modified files, retain applicable copyright, patent, trademark, and
attribution notices, and preserve any supplied `NOTICE` attributions.

The upstream license and applicable notices are retained, and modified files
carry change notices. No upstream `NOTICE` file was present at the base revision.
Memvid attribution identifies the source project and does not imply endorsement.
