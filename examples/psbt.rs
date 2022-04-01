extern crate bitcoin;
use std::{env, process};
use std::str::FromStr;

use bitcoin::secp256k1::Secp256k1;
use bitcoin::{Transaction, OutPoint, Script, TxOut, Witness, Txid, TxIn};
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::util::bip32::DerivationPath;
use bitcoin::util::bip32::ChildNumber;
use bitcoin::secp256k1::ffi::types::AlignedType;
use bitcoin::hashes::hex::FromHex;

use bitcoin::psbt::*;

fn main() {
    // BIP: 174 2-of-3 Multisig Workflow

    // This example derives root xprv from a 32-byte seed,
    // derives the child xprv with path m/84h/0h/0h,
    // Run this example with cargo and seed(hex-encoded) argument:
    // cargo run --example psbt cUkG8i1RFfWGWy5ziR11zJ5V4U4W3viSFCfyJmZnvQaUsd1xuF3T

    // 1. Use BIP32 wallet APIs to derive keys
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
    // should be "tprv8ZgxMBicQKsPd9TeAdPADNnSyH9SSUUbTVeFszDE23Ki6TBB5nCefAdHkK8Fm3qMQR6sHwA56zqRmKmxnHk37JkiFzvncDqoKmPWubu7hDF"
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


    let constructor_psbt = PartiallySignedTransaction {
        unsigned_tx: Transaction {
            version: 2,
            lock_time: 1257139,
            output: vec![
                TxOut {
                    value: 149990000,
                    script_pubkey: Script::from_hex(
                        "0014d85c2b71d0060b09c9886aeb815e50991dda124d"
                    ).unwrap(),
                },
                TxOut {
                    value: 100000000,
                    script_pubkey: Script::from_hex(
                    "001400aea9a2e5f0f876a588df5546e8742d1d87008f"
                    ).unwrap(),
                },
            ],
            input: vec![TxIn {
                previous_output: OutPoint {
                    txid: Txid::from_hex(
                        "75ddabb27b8845f5247975c8a5ba7c6f336c4570708ebe230caf6db5217ae858",
                    ).unwrap(),
                    vout: 0,
                },
                script_sig: Script::new(),
                sequence: 4294967294,
                witness: Witness::default(),
            },
            TxIn {
                previous_output: OutPoint {
                    txid: Txid::from_hex(
                        "1dea7cd05979072a3578cab271c02244ea8a090bbb46aa680a65ecd027048d83",
                    ).unwrap(),
                    vout: 1,
                },
                script_sig: Script::new(),
                sequence: 4294967294,
                witness: Witness::default(),
            }],
        },
        xpub: Default::default(),
        version: 0,
        proprietary: Default::default(),
        unknown: Default::default(),
        inputs: vec![Input::default()],
        outputs: vec![Output::default(), Output::default()],
    };
    // 2. Use the sighash APIs to compute the signature hashes

    // 3. Use all Psbt supported APIs

    // 4. Identify the API pain-points and suggest improvements.

}


