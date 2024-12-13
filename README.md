[中文说明](README_zh.md)

# genBTCAddresss

`genBTCAddresss` is an example offline Bitcoin tool that demonstrates how to:

- Use high-quality randomness to generate BIP39 mnemonics and seeds
- Use BIP32 to derive hierarchical deterministic (HD) keys from a seed
- Derive multiple Bitcoin address formats (such as P2PKH and Bech32/P2WPKH) from the same private key
- Operate entirely offline, ensuring the security of private keys
- Link to a locally installed latest version of the `libsecp256k1` C library on Ubuntu 24.04
- Use Rust and the `rust-secp256k1` crate to interface with `libsecp256k1`

**Note:** The `rust-secp256k1` crate and the `libsecp256k1` C library are different projects. The version of `rust-secp256k1` (e.g. 0.27) does not correspond to the version of the `libsecp256k1` C library (e.g. v0.6.0). Please refer to each project’s respective releases.

## Features

1. **High-quality randomness:**  
   Uses `rand::rngs::OsRng` for operating system-level high-entropy random numbers, ensuring secure key generation.

2. **Offline operation:**  
   The entire key and address generation process requires no internet connection, suitable for cold wallets or isolated environments.

3. **BIP39 mnemonics & BIP32 HD wallets:**

   - **BIP39:** Generate human-readable mnemonic phrases (e.g., 12 English words) from entropy, making backup and restoration easier.
   - **BIP32:** Derive a hierarchical deterministic tree of keys from a single seed. With just one backup of the mnemonic, you can recover all keys in the HD wallet structure.

4. **Multiple address formats:**

   - **P2PKH** (Legacy, starting with `1`)
   - **P2WPKH** (Bech32, starting with `bc1q`)

   The same private key can produce various address formats to meet different wallet and fee optimization needs.

5. **System-installed `libsecp256k1`:**  
   This example uses the `external` feature of the `rust-secp256k1` crate to link against the system-installed `libsecp256k1` library. You must have compiled and installed the latest `libsecp256k1` C library (e.g. v0.6.0) on Ubuntu 24.04 beforehand.

   Note: The `rust-secp256k1` crate version (e.g., 0.27) and the `libsecp256k1` C library version (e.g., v0.6.0) are maintained separately and are not directly correlated.

## Requirements

- Ubuntu 24.04 server
- `build-essential`, `autoconf`, `automake`, `libtool`, `pkg-config`, `git`
- A Rust development environment (`rustup` installation)
- The latest version of `libsecp256k1` (C library)
- No network connection required; can run entirely offline

## Installation Steps

1. **Install base build dependencies:**

   ```bash
   sudo apt update
   sudo apt install -y build-essential autoconf automake libtool pkg-config git
   ```

2. **Compile and install the latest `libsecp256k1`:**

   ```bash
   git clone https://github.com/bitcoin-core/secp256k1.git
   cd secp256k1
   ./autogen.sh
   ./configure --enable-module-recovery --enable-experimental --enable-module-ecdh
   make
   sudo make install
   ```

   Configure `PKG_CONFIG_PATH` so that Rust can locate `libsecp256k1`:

   ```bash
   export PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH"
   ```

3. **Install or update Rust:**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

4. **Clone and build this project:**

   ```bash
   git clone https://example.com/genBTCAddresss.git
   cd genBTCAddresss
   export RUST_SECP256K1_NO_VENDOR=1
   cargo build --release
   ```

5. **Run the tool (in an offline environment):**
   ```bash
   ./target/release/genBTCAddresss
   ```

## Usage Example

After running the tool, it will output:

- A BIP39 mnemonic phrase (e.g., `abandon abandon abandon ...`)
- A WIF-formatted private key (importable into most Bitcoin wallets)
- A P2PKH address (legacy format)
- A P2WPKH Bech32 address (modern format)

Example output:

```
Mnemonic: abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
Private Key (WIF): L4abcxyz...
P2PKH Address: 1AbcXyz...
P2WPKH (Bech32) Address: bc1qxyz...
```

Ensure that this is done in a secure, isolated environment and record the outputs carefully. Only a mnemonic backup is needed to restore the entire HD wallet structure at a later time.

## FAQs

- **Why does `rust-secp256k1` not match the `libsecp256k1` version?**  
  `rust-secp256k1` is a Rust binding, and its versioning is independent from the C library’s versioning. Ensure you specify the correct crate version in `Cargo.toml` and use the `external` feature to link to the system-installed `libsecp256k1`.

- **Can the same private key produce multiple address formats?**  
  Yes. The same private key, when processed through different hashing and encoding steps, can yield P2PKH, P2WPKH, and other address types. This flexibility allows for multiple address options from a single root key.

- **What are BIP32 and BIP39?**
  - **BIP39:** Defines how to convert entropy into a human-readable mnemonic phrase, making backup and restoration more user-friendly.
  - **BIP32:** Defines hierarchical deterministic keys, allowing an entire wallet’s keys to be recovered from a single seed.

## Future Improvements

- Add support for more address formats (e.g. P2WSH), multisig setups, or different BIP derivation paths.
- Introduce command-line parameters for flexible customization of derivation paths, output counts, or key types.
- Add unit tests and CI to ensure code reliability and compatibility.

## License

MIT License

---

This README provides a comprehensive, English-language guide summarizing all the details discussed in the previous dialogue and instructions.
