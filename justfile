# Run all checks and tests
@all: check test lint check-format

# Run check
@check:
    cargo check --all-features

# Run static analyzis
@lint:
    cargo clippy --all-features

# Check coding style
@check-format:
    cargo fmt --check

# Run all unit tests and feature combinations
@test:
    cargo test
    cargo test --features base64
    cargo test --features serde
    cargo test --features openapi
    cargo test --features base64,serde
    cargo test --features base64,openapi
    cargo test --features eq
    cargo test --all-features
