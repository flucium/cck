# CCK
<img src="./image.png" width=200 heigth=400>

## What is CCK?

## Features
In recent years, OpenSSL or its distributions have been installed by default on many operating systems, including macOS.

CCK does not depend on OpenSSL. Therefore, it does not conflict with the version of OpenSSL required by the OS or other applications.

Naturally, it will not contaminate OpenSSL or its distribution, which is installed by default on the OS.

CCK adopts and uses only libraries implemented in pure Rust as its backend.

## Backend
- **Rust Crypto**<br>
[https://github.com/RustCrypto/](https://github.com/RustCrypto/)

- **BLAKE3**<br>
[https://github.com/BLAKE3-team/BLAKE3/](https://github.com/BLAKE3-team/BLAKE3/)

- **dalek cryptography**<br>
[https://github.com/dalek-cryptography](https://github.com/dalek-cryptography)

<br>

# Build
**build**
```bash
# cargo build command is executed.
zsh ./build.sh
```

<br>

**Release build**
```bash
# cargo build --release and build target is compressed(tar -zcvf).
# The file name of the compressed directory will be in the format cck-{version}-{os}-{arch}.tar.gz
zsh ./build.sh release
```

<br>

# Install
```bash

```

<br>

# License
- **CCK:** [MIT License](https://github.com/flucium/cck/blob/main/LICENSE)

- **Libraries:** *For external libraries that CCK uses (and depends on), please follow their respective licenses and terms of use.*

<br>


# Sponsors
- **Kemo0513**<br>
[https://github.com/Kemo0513](https://github.com/Kemo0513)

- **___**<br>

- **___**<br>

- **___**<br>