#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

clone-dir := "./tmp/api"
proto-dest-dir := "./proto"

# Show available commands
default:
    @just --list --justfile {{justfile()}}

# Generate the library
generate: clone copy-buf-files export
    cargo build

# Clone the api repo
clone:
    rm -rf {{ clone-dir }}
    git clone https://github.com/openfga/api.git {{ clone-dir }}

# Copy the dependencies from the cloned repo to the root directory
copy-buf-files:
    rm -f ./buf.gen.yaml
    rm -f ./buf.yaml
    cp {{ clone-dir }}/buf.gen.yaml ./buf.gen.yaml
    cp {{ clone-dir }}/buf.yaml ./buf.yaml

[private]
export:
    mkdir -p {{ proto-dest-dir }}
    buf export {{ clone-dir }} --output {{ proto-dest-dir }}

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
