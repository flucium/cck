cargo -q bench --package cck-hash --bench hex -- --exact --nocapture && \
cargo -q bench --package cck-hash --bench base64ct -- --exact --nocapture && \
cargo -q bench --package cck-hash --bench pem -- --exact --nocapture