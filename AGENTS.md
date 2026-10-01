# Working in aimemo

This repository is an unofficial Apache-2.0 fork of Memvid. [README.md](README.md)
is the canonical guide for fork behavior, integration, verification, and licensing.

## Scope and compatibility

- Maintain `memvid-core` and its `memvid_core` import name for compatibility.
- The local capacity policy lives in `src/memvid/mutation.rs`:
  `capacity_limit()` returns `u64::MAX`, including for files with persisted tickets.
- Preserve the `.mv2` format, ticket serialization and signature checks, data
  validation, locking, write-ahead logging, and recovery behavior.
- Capacity statistics intentionally use the unlimited value. They do not measure
  available disk space. Do not change tier metadata to simulate a paid entitlement.
- Upstream SDKs and CLIs require native rebuilds to use this core. Do not claim
  that published packages or hosted-service quotas changed.
- Keep unrelated host applications and neighboring checkouts outside task scope.

## Maintenance and validation

- Read the affected code and existing tests before changing behavior.
- Keep this crate's standalone Cargo workspace so it builds when nested in another
  repository. Use the pinned Rust 1.90 toolchain with rustfmt and Clippy.
- The `collapsible_if` lint exception preserves upstream control-flow style with
  the Rust 1.90 MSRV. Scalar distance helpers retain the SIMD API's must-use contract.
- Follow the scoped build, lint, format, and capacity regression commands in
  README.md. Run relevant lifecycle, integrity, or ticket tests when those paths
  change. Expand testing only for a concrete unresolved concern.
- Keep the tracked `Cargo.lock` reproducible and preserve its modification notice
  if Cargo regenerates it. Do not silently change unrelated dependencies.
- Keep the regression proving writes above 50 MiB, reopen/append persistence,
  and ignored persisted capacity. Validate the features used by the consumer.
- Store build outputs, test memory files, caches, and detailed local QA logs in
  ignored locations. Do not commit generated binaries or private application data.
- Keep current usage and behavioral facts in README.md. Do not add duplicate
  handoff guides, historical plans, or per-folder summaries.
- Use local validation by default. Do not enable or dispatch hosted workflows,
  publish packages, or change account settings without an applicable user request.

## Attribution

Retain LICENSE and all applicable upstream copyright, patent, trademark, and
attribution notices. Mark every modified upstream file with a prominent notice
identifying the change, as required by Apache-2.0 Section 4(b). Preserve supplied
NOTICE attributions if an upstream update introduces them. Keep the fork's
unofficial status clear; upstream branding does not imply endorsement.
