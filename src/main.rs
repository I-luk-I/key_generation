
use elliptic_curve::rand_core::{OsRng, RngCore};
use secp256k1::{key::SecretKey, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};
use ripemd::{Ripemd160, Digest as RipemdDigest};
use hex;

fn main() {
    let mut sk_bytes = [0u8; 32];
    let mut rng = OsRng;
    rng.fill_bytes(&mut sk_bytes);

    println!("private_key: {}", hex::encode(&sk_bytes));

    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&sk_bytes)
        .expect("Error creating secret key from slice");

    let pubkey = PublicKey::from_secret_key(&secp, &sk);

    println!("public key: {}", hex::encode(pubkey.serialize()));

    let sha = Sha256::digest(&pubkey.serialize());

    let addr_raw = Ripemd160::digest(&sha);

    let mut hasher_checksum = Sha256::new();
    hasher_checksum.update(&addr_raw);
    let checksum = &hasher_checksum.finalize()[..4];

    let addr = [&addr_raw[..], checksum].concat();

    println!("address: {}", hex::encode(addr));
}