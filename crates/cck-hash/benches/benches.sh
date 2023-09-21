cargo -q bench --package cck-hash --bench blake3 -- --exact --nocapture && \
cargo -q bench --package cck-hash --bench sha2 -- --exact --nocapture