# Change Request: ARM64 Build Issues with `simd-kernels` and `minarrow`

**Date:** 2025-08-28

**Status:** Open

**Affected Components:**
*   `simd-kernels` crate
*   `minarrow` crate (dependency of `simd-kernels`)
*   ARM64 architecture builds

**Problem Description:**
Attempts to build the `simd-kernels` crate on ARM64 architecture consistently fail due to two primary issues originating from its `minarrow` dependency:
1.  **Unstable Features (E0554):** `minarrow` utilizes unstable Rust features (`allocator_api`, `slice_ptr_get`) which are not permitted on the stable release channel.
2.  **FFI Type Mismatches (E0308):** There are type mismatches in the C Foreign Function Interface (FFI) within `minarrow` (specifically `*const i8` vs `*const u8`), indicating an incompatibility with how C strings are handled.

Investigation revealed that `minarrow` version `0.1.7` is the latest available on `crates.io`, and these issues persist in that version.

**Proposed Resolution:**
To unblock ARM64 builds and maintain project stability, `simd-kernels` has been temporarily excluded from the top-level workspace build. This was achieved by removing its entry from the `members` section of `monomcp/Cargo.toml`. The `simd-kernels` directory itself has been retained as per user instruction.

**Verification:**
Successful top-level `cargo build --release` on ARM64 without `simd-kernels` included.

**Impact:**
*   **Positive:** Enables successful compilation of the main `monomcp` workspace on ARM64.
*   **Negative:** `simd-kernels` functionality is currently unavailable on ARM64 builds.

**Next Steps/Follow-up:**
*   Investigate long-term solutions for `minarrow` compatibility on stable Rust and ARM64 (e.g., contributing fixes upstream to `minarrow`, finding an alternative library, or re-evaluating the need for `simd-kernels` on ARM64).
*   Re-evaluate inclusion of `simd-kernels` once `minarrow` issues are resolved or a suitable alternative is found.
