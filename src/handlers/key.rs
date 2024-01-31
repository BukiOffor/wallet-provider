use std::io::{Error,ErrorKind};
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::{
    network::Network,
    secp256k1::Secp256k1,
    bip32::{DerivationPath, Xpriv, Xpub},
};

#[allow(unused_variables)]
fn generate_root_key() -> Result<Xpriv,Error> {
    let (mnemonic,seed) = get_seed();
    //let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    let root_key = Xpriv::new_master(Network::Bitcoin, seed.as_bytes()).expect("Valid seed");
    println!("Root Private key 0x{}\n",hex::encode(root_key.to_priv().to_bytes()));
    Ok(root_key)
}

fn get_eth_path(){
     // bitcoin library uses the secp256k1 C library which requires a context
     let root_key = generate_root_key().expect("something went wrong");
     let ctx = Secp256k1::new();
     let derived = root_key.derive_priv(&ctx, 
         &"m/44'/60'/0'/0/0".parse::<DerivationPath>().expect("Valid path"),)
         .expect("Valid derive");
     println!("Private key 0x{}\n", hex::encode(derived.to_priv().to_bytes()));
}

fn get_seed() -> (Mnemonic, Seed) {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let seed = Seed::new(&mnemonic, "");
    println!("{:?}", mnemonic);
    (mnemonic, seed)
}