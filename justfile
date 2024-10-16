#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

clone-dir := "./tmp/api"
proto-dest-dir := "./proto"

# Show available commands
default:
    @just --list --justfile {{justfile()}}

# Update the generated rust code from the protobuf files
update: 
    buf generate https://github.com/openfga/api#format=git

# Run cargo doc
doc $RUSTDOCFLAGS="-D warnings":
    cargo doc --all --no-deps

# Run cargo doc on all crates and open the docs in your browser
doc-open $RUSTDOCFLAGS="-A missing_docs":
    cargo doc --all --no-deps --open

# Substitute BIN for your bin directory.
# Substitute VERSION for the current released version.
install-buf:
    #!/usr/bin/env sh
    BIN="/usr/local/bin" && \
    VERSION="1.30.1" && \
    curl -sSL \
    "https://github.com/bufbuild/buf/releases/download/v${VERSION}/buf-$(uname -s)-$(uname -m)" \
    -o "${BIN}/buf" && \
    chmod +x "${BIN}/buf"

[private]
fmt:
    cargo +nightly fmt --all

# Show unused dependencies
udeps:
    cargo +nightly udeps

# Run various auditing tools to assure we are legal and safe
audit:
    cargo deny check advisories bans licenses sources

# Run cargo clippy on all crates, fixing what can be fixed, and format all code
clippy-fix:
    cargo clippy --fix --all --tests --examples
    cargo clippy --fix --allow-dirty --all
    cargo fmt --all
