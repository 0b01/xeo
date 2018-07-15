use errors::MDCResult;
use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};

struct Crypto {
    rsa: Rsa<Private>,
}

impl Crypto {
    fn new() -> Self {
        let rsa = Rsa::generate(2048).unwrap();
        Crypto { rsa }
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
        let data = vec![1; 128];
        let enc = c.encrypt_pub(&data);
        let dec = c.decrypt_priv(&enc);
        assert!(dec == data);
    }
}
