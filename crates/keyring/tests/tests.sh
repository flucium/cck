cargo -q test --package keyring --test expiry --  --nocapture && \
cargo -q test --package keyring --test key_type --  --nocapture && \
cargo -q test --package keyring --test key --  --nocapture && \
cargo -q test --package keyring --test string --  --nocapture