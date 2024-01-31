use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand::{thread_rng,prelude::ThreadRng};

pub fn generate_rsa_keys(){
    let mut rng = thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
}
       

pub fn encrypt(pub_key:RsaPublicKey, mut rng:ThreadRng) -> Vec<u8> {
    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    enc_data
}

pub fn decrypt (priv_key:RsaPrivateKey, enc_data:Vec<u8>) -> Vec<u8>{
    // Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    dec_data
}
