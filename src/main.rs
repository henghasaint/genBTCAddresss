use base58::ToBase58;
use bip39::{Language, Mnemonic, Seed};
use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::{sha256, Hash};
use ripemd::Ripemd160;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

fn generate_mnemonic() -> Mnemonic {
    Mnemonic::generate_in(Language::English, 12).unwrap()
}

fn mnemonic_to_seed(mnemonic: &Mnemonic, passphrase: &str) -> Vec<u8> {
    Seed::new(mnemonic, passphrase).as_bytes().to_vec()
}

fn private_key_to_wif(private_key: &SecretKey) -> String {
    let mut wif = Vec::new();
    wif.push(0x80); // Mainnet prefix
    wif.extend(private_key.secret_bytes());

    let checksum = sha256::Hash::hash(&sha256::Hash::hash(&wif)).to_byte_array();
    wif.extend(&checksum[0..4]);

    wif.to_base58()
}

fn public_key_to_address(public_key: &PublicKey) -> String {
    let sha256_hash = Sha256::digest(&public_key.serialize());
    let ripemd_hash = Ripemd160::digest(&sha256_hash);

    let mut address = Vec::new();
    address.push(0x00); // Mainnet prefix for P2PKH
    address.extend(&ripemd_hash);

    let checksum = Sha256::digest(&Sha256::digest(&address));
    address.extend(&checksum[0..4]);

    address.to_base58()
}

fn main() {
    let secp = Secp256k1::new();

    // Step 1: Generate Mnemonic
    let mnemonic = generate_mnemonic();
    println!("Generated Mnemonic: {}", mnemonic.phrase());

    // Step 2: Convert Mnemonic to Seed
    let seed = mnemonic_to_seed(&mnemonic, "");

    // Step 3: Generate Private Key
    let private_key = SecretKey::from_slice(&seed[0..32]).expect("32 bytes, within curve order");
    println!("Private Key (WIF): {}", private_key_to_wif(&private_key));

    // Step 4: Generate Public Key
    let public_key = PublicKey::from_secret_key(&secp, &private_key);
    println!("Public Key: {}", public_key);

    // Step 5: Generate BTC Address
    let btc_address = public_key_to_address(&public_key);
    println!("BTC Address: {}", btc_address);
}
