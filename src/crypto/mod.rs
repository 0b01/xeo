use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};

struct Crypto {
    rsa: Rsa<Private>,
}

impl Crypto {
    fn new() -> Self {
        let rsa = Rsa::generate(4096).unwrap();
        Crypto { rsa }
    }

    fn get_pubkey(&self) -> Vec<u8> {
        self.rsa.public_key_to_der().unwrap()
    }

    fn encrypt_pub(&self, data: &[u8]) -> Vec<u8> {
        let mut encrypted_data: Vec<u8> = vec![0; 512];
        let padding = Padding::PKCS1;
        let sz = self
            .rsa
            .public_encrypt(&data, encrypted_data.as_mut_slice(), padding)
            .unwrap();
        encrypted_data.resize(sz, 0);
        encrypted_data
    }

    fn decrypt_priv(&self, data: &[u8]) -> Vec<u8> {
        let mut decrypted_data: Vec<u8> = vec![0; 512];
        let padding = Padding::PKCS1;
        let sz = self
            .rsa
            .private_decrypt(&data, decrypted_data.as_mut_slice(), padding)
            .unwrap();
        decrypted_data.resize(sz, 0);
        decrypted_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enc_dec() {
        let c = Crypto::new();
        let data = vec![1; 256];
        let enc = c.encrypt_pub(&data);
        let dec = c.decrypt_priv(&enc);
        assert!(dec == data);
    }

    #[test]
    fn test_get_pubkey() {
        // let c = Crypto::new();
        // let pubkey = c.get_pubkey();
        // println!("{:?}", pubkey);
    }
}
