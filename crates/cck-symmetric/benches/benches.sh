cargo -q bench --features=alloc --package cck-symmetric --bench aes -- --exact --nocapture && \
cargo -q bench --features=alloc --package cck-symmetric --bench chacha -- --exact --nocapture