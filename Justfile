default: build-dev


alias b := build-dev
alias i := install-dev
alias l := lint-dev


export RUST_BACKTRACE := "1"
export RUST_LOG := "debug"


cli := "openproject-cli"
lib := "libOpenProject.so"
target_dev := if os() == "linux" {
    "x86_64-unknown-linux-gnu"
} else if os() == "mac" {
    "universal2-apple-darwin"
} else {
    error("Unsupported OS: {{ os() }}; only supporting {{ supported_os }}")
}
# TODO: replace with command in backticks; figure out why x86_64-unknown-linux-gnu breaks
# https://just.systems/man/en/chapter_33.html#command-evaluation-using-backticks
target_release := """
    aarch64-apple-darwin
    aarch64-unknown-linux-gnu
    universal2-apple-darwin
    x86_64-apple-darwin
    x86_64-unknown-linux-gnu
"""
zigbuild := path_exists(home_directory()/".cargo/bin/cargo-zigbuild")


set ignore-comments
set shell := ["bash", "-c"]


# build for the local machine
build-dev: prep setup lint-dev
    @echo "Compiling development binaries for target: {{ target_dev }}"
    cargo build --target {{ target_dev }}

# build for all supported targets in docker
build: setup
    @echo "Cross-compiling binaries..."
    @for target in $(echo "{{ target_release }}" | sed "s/\n/ /g"); do \
        echo "Compiling release binaries for target: ${target}"; \
        cargo zigbuild --release --target ${target}; \
    done
    echo "Compilation completed";


# install the cli and library to the local machine
install-dev: build-dev
    # https://tldp.org/HOWTO/Program-Library-HOWTO/shared-libraries.html#:~:text=3.1.1.%20Shared%20Library%20Names
    install -Dm755 target/{{ cli }} /bin/{{ cli }}
    install -Dm755 target/{{ lib }} /usr/lib/{{ lib }}

check:
    cargo fmt -- --check

lint:
    cargo clippy --all-targets --all-features -- -D warnings

# lint with pedantic warnings
lint-dev: check
    cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic
    cargo clippy --fix

# Install rustup if rustc or cargo are not installed
prep:
    @if command -v rustc &> /dev/null || command -v cargo &> /dev/null; then \
        echo "Rustc and Cargo are already installed"; \
    else \
        echo "Installing rustup (rustc & cargo)"; \
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; \
    fi

publish: test
    cargo publish

# build and publish to crates.io
release: build publish

# install rustup, rustc, cargo, and cross-compilation targets
setup:
    @echo "Setup"
    @for target in $(echo "{{ target_release }}" | sed "s/\n/ /g"); do \
        if [[ ${target} =~ "universal" ]]; then \
            echo "Skipping toolchain installation for: ${target}"; \
            continue; \
        fi; \
        echo "Adding toolchain and build target: ${target}"; \
        rustup toolchain install --force-non-host stable-${target} --component clippy; \
        rustup target add --toolchain stable-${target} ${target}; \
        echo "Finished ${target}" && echo ""; \
    done

test:
    cargo test

zigbuild:
    echo "Installing cargo-zigbuild"
    @if ! {{ zigbuild }}; then \
        echo "Installing cargo-zigbuild"; \
        cargo install cargo-zigbuild; \
    fi
