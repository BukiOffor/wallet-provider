use std::error::Error;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::{
    network::Network,
    secp256k1::Secp256k1,
    bip32::{DerivationPath, Xpriv, Xpub},
};
use serde::Deserialize;
use crate::utils::{errors::KeyError, types::KeyMap};



#[allow(unused_variables)]
fn generate_root_key() -> Result<Xpriv, Box<dyn Error>> {
    let (mnemonic,seed) = get_seed();
    //let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    let root_key = Xpriv::new_master(Network::Bitcoin, seed.as_bytes()).expect("Valid seed");
    println!("Root Private key 0x{}\n",hex::encode(root_key.to_priv().to_bytes()));

    Ok(root_key)
}


fn get_eth_path(path:&str)-> Result<String,Box<dyn Error>>{
     // bitcoin library uses the secp256k1 C library which requires a context
     let root_key = generate_root_key().expect("something went wrong");
     let ctx = Secp256k1::new();
     let derived = root_key.derive_priv(&ctx, 
         &path.parse::<DerivationPath>().expect("Valid path"),)
         .expect("Valid derive");
    let secret = hex::encode(derived.to_priv().to_bytes());
    let chaincode = hex::encode(root_key.chain_code.as_bytes());
    let fingerprint = hex::encode(root_key.fingerprint(&ctx));
    let keymap = KeyMap::new(path.to_owned(),derived.depth,secret,&fingerprint,chaincode);
    Ok(serde_json::to_string_pretty(&keymap)?)
}

fn get_seed() -> (Mnemonic, Seed) {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let seed = Seed::new(&mnemonic, "");
    println!("{:?}", mnemonic);
    (mnemonic, seed)
}