extern crate bitcoin;

use std::{env, process};
use std::str::FromStr;

use bitcoin::secp256k1::Secp256k1;
use bitcoin::PublicKey;
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::util::bip32::DerivationPath;
use bitcoin::util::bip32::ChildNumber;
use bitcoin::util::address::Address;
use bitcoin::secp256k1::ffi::types::AlignedType;
use bitcoin::hashes::hex::FromHex;

fn main() {
    // BIP: 174 2-of-3 Multisig Workflow

    // This example derives root xprv from a 32-byte seed,
    // derives the child xprv with path m/84h/0h/0h,
    // Run this example with cargo and seed(hex-encoded) argument:
    // cargo run --example psbt 7934c09359b234e076b9fa5a1abfd38e3dc2a9939745b7cc3c22a48d831d14bd

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("not enough arguments. usage: {} <hex-encoded 32-byte seed>", &args[0]);
        process::exit(1);
    }

    let seed_hex = &args[1];
    println!("Seed: {}", seed_hex);

    // default network as mainnet
    let network = bitcoin::Network::Bitcoin;
    println!("Network: {:?}", network);

    let seed = Vec::from_hex(seed_hex).unwrap();

    // we need secp256k1 context for key derivation
    let mut buf: Vec<AlignedType> = Vec::new();
    buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    // calculate root key from seed
    let root = ExtendedPrivKey::new_master(network, &seed).unwrap();

    // derive child xpub
    let path = DerivationPath::from_str("m/84h/0h/0h").unwrap();
    let child = root.derive_priv(&secp, &path).unwrap();
    let xpub = ExtendedPubKey::from_priv(&secp, &child);

    // generate first receiving address at m/0/0
    // manually creating indexes this time
    let zero = ChildNumber::from_normal_idx(0).unwrap();
    let a_key = xpub.derive_pub(&secp, &vec![zero, zero])
        .unwrap()
        .public_key;

    // generate first receiving address at m/1/0
    // manually creating indexes this time
    let one = ChildNumber::from_normal_idx(1).unwrap();
    let c_key = xpub.derive_pub(&secp, &vec![one, zero])
        .unwrap()
        .public_key;

    // generate first receiving address at m/2/0
    // manually creating indexes this time
    let two = ChildNumber::from_normal_idx(2).unwrap();
    let c_key = xpub.derive_pub(&secp, &vec![two, zero])
        .unwrap()
        .public_key;

    // 1. Use BIP32 wallet APIs to derive all priv ⭕️/public ✅ keys


}
