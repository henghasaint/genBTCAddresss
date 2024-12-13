# genBTCAddresss

`genBTCAddresss` 是一个示例性的离线比特币工具，展示如何在 Ubuntu 24.04 环境下：

- 使用高质量随机数生成 BIP39 助记词和种子
- 使用 BIP32 从种子中派生出分层确定性(HD)密钥
- 在同一私钥基础上输出多种格式的比特币地址（如 P2PKH 和 Bech32 (P2WPKH)）
- 全程离线运行，确保私钥不会泄露
- 调用已安装在系统上的 `libsecp256k1` 最新版本(C 库)
- 使用 Rust 和 `rust-secp256k1` crate 来链接 `libsecp256k1`。  
  **注意**：`rust-secp256k1` 与 `libsecp256k1` 是不同的项目，前者是 Rust 对后者的绑定库。`rust-secp256k1` 的版本（例如 0.27）与 `libsecp256k1` 的 C 库版本（例如 v0.6.0）不是一一对应的，请以各自项目的发布版本为准。

## 功能特性

1. **高质量随机数**：通过 `rand::rngs::OsRng` 获取操作系统级高熵源，保障私钥生成的安全性。

2. **离线运行**：整个私钥和地址生成过程不需要网络访问，适合在“冷钱包”或隔离环境中执行。

3. **BIP39 助记词 & BIP32 HD 钱包支持**：

   - 使用 BIP39 标准从高质量熵生成一组助记词（如 12 个英文单词），方便人类可读和备份。
   - 使用 BIP32 标准根据种子派生分层确定性密钥，只需备份助记词即可恢复整个 HD 钱包结构和所有子密钥。

4. **多种地址格式支持**：

   - P2PKH（传统地址，以`1`开头）
   - P2WPKH（Bech32 地址，以`bc1q`开头）

   同一个私钥可以导出多种类型的地址，以满足不同钱包和费用优化策略的需求。

5. **系统安装的 `libsecp256k1`**：  
   本示例使用 Rust crate `rust-secp256k1` 的 `external` 特性链接系统安装的 `libsecp256k1`。这意味着你需要提前在 Ubuntu 24.04 上编译安装 `libsecp256k1` C 库的最新版本（例如 v0.6.0）。

   注：`rust-secp256k1` crate 的版本号（如 0.27）与 `libsecp256k1` C 库版本号（如 v0.6.0）并不直接对应。`rust-secp256k1` 是对 `libsecp256k1` 的绑定和封装，通过 FFI 调用系统库的函数。

## 环境要求

- Ubuntu 24.04 服务器
- `build-essential`, `autoconf`, `automake`, `libtool`, `pkg-config`, `git` 等构建工具
- Rust 开发环境（`rustup` 安装）
- 最新版本的 `libsecp256k1` (C 库)
- 无网络需求，可在离线环境下执行

## 安装步骤

1. **安装基础构建依赖：**

   ```bash
   sudo apt update
   sudo apt install -y build-essential autoconf automake libtool pkg-config git
   ```

2. **编译安装最新版本的 libsecp256k1：**

   ```bash
   git clone https://github.com/bitcoin-core/secp256k1.git
   cd secp256k1
   ./autogen.sh
   ./configure --enable-module-recovery --enable-experimental --enable-module-ecdh
   make
   sudo make install
   ```

   配置 `PKG_CONFIG_PATH`，确保 Rust 程序能找到 `libsecp256k1`：

   ```bash
   export PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH"
   ```

3. **安装或更新 Rust：**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

4. **克隆并编译本项目：**

   ```bash
   git clone https://example.com/genBTCAddresss.git
   cd genBTCAddresss
   cargo build --release
   ```

5. **运行工具（在离线环境中）：**
   ```bash
   ./target/release/genBTCAddresss
   ```

## 使用示例

运行后程序将输出：

- 助记词（如 `abandon abandon abandon ...`）
- WIF 格式的私钥（易导入到标准比特币钱包）
- P2PKH 地址（传统格式地址）
- P2WPKH 地址（Bech32 格式地址）

示例输出：

```
Mnemonic: abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
Private Key (WIF): L4abcxyz...
P2PKH Address: 1AbcXyz...
P2WPKH (Bech32) Address: bc1qxyz...
```

请务必在安全隔离的环境中运行并记录输出。只需妥善保存助记词，即可日后恢复整个 HD 钱包结构中的所有私钥和地址。

## 常见问题

- **rust-secp256k1 与 libsecp256k1 版本差异？**  
  `rust-secp256k1` 是 Rust 绑定库，其版本号（如 0.27）与 C 库 `libsecp256k1`（如 v0.6.0）独立维护。你需要在 `Cargo.toml` 中指定 `rust-secp256k1` crate 版本，同时使用 `external` 特性确保它链接到系统安装的 `libsecp256k1`。

- **同一私钥是否可对应多种地址格式？**  
  是的。同一私钥可根据不同的哈希和编码过程生成 P2PKH、P2WPKH 及其他格式地址。这样在同一根密钥下，可以创建不同类型的地址以满足不同需求。

- **BIP32、BIP39 是什么？**
  - **BIP39**：定义了将随机熵转换为人类可读助记词的标准，使用户更容易备份和恢复钱包。
  - **BIP32**：定义分层确定性密钥结构，可从一个种子派生出无数子密钥，只需备份一次即可恢复整个钱包。

## 后续迭代

- 添加对更多地址格式（如 P2WSH）、多重签名或其他 BIP 路径的支持。
- 为脚本增加 CLI 参数，以便灵活指定派生路径、输出数量或密钥类型。
- 增加测试用例和 CI 来确保代码质量和兼容性。

## 许可证

MIT License

---
