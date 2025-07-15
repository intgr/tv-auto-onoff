set dotenv-load := true

default: validate

validate: fmt test clippy

clippy:
    cargo clippy --color=always --all-targets --all-features -- --no-deps -D warnings

test:
    cargo test --color=always --all-targets

fmt:
    cargo fmt -- --color=always --check

run:
    cargo run -- $TV_IP_ADDRESS

install:
    cargo install --path .
    test -f ~/.config/systemd/user/tv-auto-onoff.service  # Create this file manually, see README.md
    systemctl --user restart tv-auto-onoff.service

# Check Renovate configuration
check-renovate:
    npx --yes --package renovate -- renovate-config-validator --strict
