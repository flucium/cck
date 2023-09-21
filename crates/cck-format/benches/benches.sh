cargo -q bench --features=alloc --package cck-format --bench hex -- --exact --nocapture && \
cargo -q bench --features=alloc --package cck-format --bench base64ct -- --exact --nocapture && \
cargo -q bench --features=alloc --package cck-format --bench pem -- --exact --nocapture
