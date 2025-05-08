default: validate

validate: fmt test clippy

clippy:
    cargo clippy --color=always --all-targets --all-features -- -D warnings

test:
    cargo test

fmt:
    cargo fmt -- --color=always --check

install:
    cargo install --path .
    test -f ~/.config/systemd/user/tv-auto-onoff.service  # Create this file manually, see README.md
    systemctl --user restart tv-auto-onoff.service

# Check Renovate configuration
check-renovate:
    npx --yes --package renovate -- renovate-config-validator --strict
