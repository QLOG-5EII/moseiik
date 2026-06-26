# Software Quality Report — Moseiik

## Objective

The objective of the project is to improve the software quality of Moseiik by adding unit tests, integration tests, a reproducible Docker environment, and a multi-architecture continuous integration pipeline.

## Unit Tests

We tested the following functions:

- `l1_generic`
- `l1_x86_sse2`
- `l1_neon`
- `prepare_tiles`
- `prepare_target`

### `l1_generic`

This test verifies that the L1 distance between two images is correctly computed. An image compared with itself must return a distance of 0. Two different images return a manually calculated distance.

### `l1_x86_sse2`

This test verifies that the SSE2 optimized version returns the same result as the generic version on x86/x86_64 architectures. However we encountered an error message during its execution, which we couldn't solve (even with your help). Which is why this unitary test fails.
#### This error is expected:
```bash
thread 'main::tests::unit_test_x86' (5534) panicked at src/main.rs:434:9:
assertion `left == right` failed
  left: 0
 right: 150
```

### `l1_neon`

This test verifies that the NEON optimized version returns the same result as the generic version on ARM64 architecture.

### `prepare_tiles`

This test verifies that the tiles are correctly loaded and resized to the expected size.

### `prepare_target`

This test verifies that the target image is prepared with dimensions divisible by the tile size.

## Integration Tests

The integration tests run the complete program using `target-small.png` and `tiles-small`. The generated image is then compared pixel by pixel with the target image.

This test verifies the overall behavior of the application.

## Docker

A Dockerfile was added to run the tests in a reproducible environment. Which works fine using the Github Agent.

Command used:

```bash
cargo test --release

## GitHub Actions

A GitHub Actions CI pipeline was added. It automatically runs the tests on:

- `linux/amd64`
- `linux/arm64`

The objective is to verify that the project works on both architectures, especially because the code contains architecture-specific SSE2 and NEON optimizations.

## Conclusion

The project now has a base of automated tests, a reproducible Docker environment, and a multi-architecture continuous integration pipeline. This makes it possible to detect regressions quickly and improve the reliability of the software.
