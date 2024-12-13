use std::str::FromStr;

use bech32::{u5, ToBase32};
use bs58;
use rand_core::OsRng; // 通过 getrandom 特性启用的 OsRng，与 bip39 的 rand_core 0.5 匹配
use secp256k1::{PublicKey, Secp256k1, SecretKey};

use bip32::secp256k1::Secp256k1; // 指定 Secp256k1 密钥材料
use bip32::{ChildNumber, DerivationPath, ExtendedPrivateKey};
use bip39::{Language, Mnemonic};
use bitcoin_hashes::{hash160, sha256, Hash};

fn main() {
    // 使用 generate_in_with + OsRng 来生成助记词
    let mut rng = OsRng;
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 12)
        .expect("Failed to generate mnemonic");

    println!("Mnemonic: {}", mnemonic);

    let seed_bytes = mnemonic.to_seed("");
    let secp = Secp256k1::new();

    // 指定 ExtendedPrivateKey 使用 Secp256k1 类型
    let mut xprv: ExtendedPrivateKey<Secp256k1> =
        ExtendedPrivateKey::new(&seed_bytes).expect("Failed to create ExtendedPrivateKey");

    let derivation_path = DerivationPath::from_str("m/44'/0'/0'/0/0").unwrap();

    // 手动遍历派生路径的每个 ChildNumber，依次派生子私钥
    for cnum in derivation_path.into_iter() {
        xprv = xprv.derive_child(cnum).expect("Failed to derive child key");
    }

    let sk = SecretKey::from_slice(&xprv.private_key().to_bytes()).expect("Invalid secret key");
    let public_key = PublicKey::from_secret_key(&secp, &sk);

    let wif = private_key_to_wif(&sk, true);
    println!("Private Key (WIF): {}", wif);

    let p2pkh_address = public_key_to_p2pkh(&public_key);
    println!("P2PKH Address: {}", p2pkh_address);

    let p2wpkh_address = public_key_to_p2wpkh(&public_key);
    println!("P2WPKH (Bech32) Address: {}", p2wpkh_address);
}

fn private_key_to_wif(sk: &SecretKey, compressed: bool) -> String {
    let sk_bytes = sk.secret_bytes();
    let mut payload = vec![0x80];
    payload.extend_from_slice(&sk_bytes);
    if compressed {
        payload.push(0x01);
    }
    let checksum = double_sha256(&payload);
    let wif_bytes = [&payload[..], &checksum[0..4]].concat();
    bs58::encode(wif_bytes).into_string()
}

fn public_key_to_p2pkh(pk: &PublicKey) -> String {
    let pk_bytes = pk.serialize();
    let hash160_pubkey = hash160::Hash::hash(&pk_bytes);

    let mut payload = vec![0x00];
    payload.extend_from_slice(&hash160_pubkey[..]);

    let checksum = double_sha256(&payload);
    let address_bytes = [&payload[..], &checksum[0..4]].concat();
    bs58::encode(address_bytes).into_string()
}

fn public_key_to_p2wpkh(pk: &PublicKey) -> String {
    let pk_bytes = pk.serialize();
    let hash160_pubkey = hash160::Hash::hash(&pk_bytes);

    let hrp = "bc";
    let mut data = Vec::new();
    data.push(u5::try_from_u8(0).unwrap());
    data.extend(hash160_pubkey.to_base32());
    bech32::encode(hrp, data, bech32::Variant::Bech32).expect("Bech32 encoding failed")
}

fn double_sha256(data: &[u8]) -> [u8; 32] {
    let first = sha256::Hash::hash(data);
    let second = sha256::Hash::hash(&first);
    let mut result = [0u8; 32];
    result.copy_from_slice(&second);
    result
}
