cargo -q bench --package cck-asymmetric --bench ed25519 -- --exact --nocapture && \
cargo -q bench --package cck-asymmetric --bench x25519 -- --exact --nocapture