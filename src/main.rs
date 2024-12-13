use bech32::{self, ToBase32};
use bs58;
use rand::rngs::OsRng;
use ripemd160::{Digest, Ripemd160};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

use bip32::{ChildNumber, DerivationPath, ExtendedKey, ExtendedPrivateKey};
use bip39::{Language, Mnemonic, Seed};

fn main() {
    // 1. 生成助记词(BIP39)
    let mut rng = OsRng;
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 12)
        .expect("Failed to generate mnemonic");
    println!("Mnemonic: {}", mnemonic);

    // 2. 从助记词获取种子 (BIP39)
    let seed = Seed::new(&mnemonic, "");

    // 3. 使用BIP32派生根扩展私钥
    let xprv = ExtendedPrivateKey::new_master(bip32::Prefix::XPRV, seed.as_bytes())
        .expect("Failed to create master key");

    // 示例派生路径：m/44'/0'/0'/0/0 (BIP44路径，用于比特币的第一个外部地址)
    // 这里可以根据需求更改路径，如 m/84'/0'/0'/0/0 用于Bech32地址。
    let derivation_path = "m/44'/0'/0'/0/0".parse::<DerivationPath>().unwrap();
    let child_xprv = xprv
        .derive_priv(&Secp256k1::new(), &derivation_path)
        .expect("Failed to derive child key");

    let secret_key = &child_xprv.private_key;
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&secret_key.to_bytes()).expect("Invalid secret key bytes");
    let public_key = PublicKey::from_secret_key(&secp, &sk);

    // 输出WIF格式私钥（对标P2PKH压缩私钥）
    let wif = private_key_to_wif(&sk, true);
    println!("Private Key (WIF): {}", wif);

    // 4. 使用同一个公钥生成多种格式地址
    // P2PKH地址（以1开头）
    let p2pkh_address = public_key_to_p2pkh(&public_key);
    println!("P2PKH Address: {}", p2pkh_address);

    // P2WPKH地址 (Bech32,以bc1q开头)
    let p2wpkh_address = public_key_to_p2wpkh(&public_key);
    println!("P2WPKH (Bech32) Address: {}", p2wpkh_address);
}

// 将私钥转换为WIF格式 (主网压缩)
fn private_key_to_wif(sk: &SecretKey, compressed: bool) -> String {
    let sk_bytes = sk.secret_bytes();
    let mut payload = Vec::with_capacity(34);
    // 主网版本前缀0x80
    payload.push(0x80);
    payload.extend_from_slice(&sk_bytes);
    if compressed {
        // 对应压缩公钥标志
        payload.push(0x01);
    }
    let checksum = double_sha256(&payload);
    let wif_bytes = [&payload[..], &checksum[0..4]].concat();
    bs58::encode(wif_bytes).into_string()
}

// 计算公钥的P2PKH地址
fn public_key_to_p2pkh(pk: &PublicKey) -> String {
    let pk_bytes = pk.serialize(); // 压缩公钥33字节
    let sha_hash = Sha256::digest(&pk_bytes);
    let ripemd_hash = Ripemd160::digest(&sha_hash);

    // 主网P2PKH版本字节 0x00
    let mut payload = Vec::with_capacity(21);
    payload.push(0x00);
    payload.extend_from_slice(&ripemd_hash);

    let checksum = double_sha256(&payload);
    let address_bytes = [&payload[..], &checksum[0..4]].concat();
    bs58::encode(address_bytes).into_string()
}

// 计算公钥的P2WPKH (Bech32) 地址
fn public_key_to_p2wpkh(pk: &PublicKey) -> String {
    let pk_bytes = pk.serialize();
    let sha_hash = Sha256::digest(&pk_bytes);
    let ripemd_hash = Ripemd160::digest(&sha_hash);

    // Bech32人类可读部分(hrp)为"bc"，并且见证版本为0
    let hrp = "bc";
    let mut data = vec![0u5]; // witness version = 0
                              // 转换为 base32
    data.extend(ripemd_hash.to_base32());
    bech32::encode(hrp, data, bech32::Variant::Bech32).expect("Bech32 encoding failed")
}

// 双重SHA256
fn double_sha256(data: &[u8]) -> [u8; 32] {
    let first = Sha256::digest(data);
    let second = Sha256::digest(&first);
    let mut result = [0u8; 32];
    result.copy_from_slice(&second);
    result
}
