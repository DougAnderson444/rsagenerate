use hmac_drbg::*;
use rand::{rngs::StdRng, SeedableRng};
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
use typenum::consts::*;

pub fn determine_bytes(seed: String) -> [u8; 32] {
    let entropy: &[u8] = seed.as_bytes();
    let nonce: &[u8] = &[];
    let pers: &[u8] = &[];

    let mut drbg = HmacDRBG::<Sha256>::new(entropy, nonce, pers);
    let b = drbg.generate::<U32>(None);

    let mut sized: [u8; 32] = Default::default(); // has to be [0..32]
    sized.copy_from_slice(b.as_slice());

    sized
}

pub fn do_rsa(seed: String) -> RsaPublicKey {
    let mut rng = StdRng::from_seed(determine_bytes(seed));
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    pub_key
}

#[cfg(test)]
mod tests {

    use super::*;
    use hex::*;

    #[test]
    fn basic() {
        let mut drbg = HmacDRBG::<Sha256>::new(
            "totally random0123456789".as_bytes(),
            "secret nonce".as_bytes(),
            "my drbg".as_bytes(),
        );
        assert_eq!(
            drbg.generate::<U32>(None).as_slice(),
            decode("018ec5f8e08c41e5ac974eb129ac297c5388ee1864324fa13d9b15cf98d9a157")
                .unwrap()
                .as_slice()
        );
    }

    #[test]
    fn null_nonce() {
        let entropy: &[u8] = "the seed value".as_bytes();
        let nonce: &[u8] = &[];
        let pers: &[u8] = &[];

        let mut drbg = HmacDRBG::<Sha256>::new(entropy, nonce, pers);
        let b = drbg.generate::<U32>(None);

        assert_eq!(
            b.as_slice(),
            hex::decode("74f6b2d13935142e0adf6b275e2fdd6a4bab20f758a2a4fe6369f94312578925")
                .unwrap()
                .as_slice()
        );
    }

    #[test]
    fn use_fn() {
        let result = determine_bytes("the seed value".to_string());

        assert_eq!(
            result,
            hex::decode("74f6b2d13935142e0adf6b275e2fdd6a4bab20f758a2a4fe6369f94312578925")
                .unwrap()
                .as_slice()
        );
    }

    #[test]
    fn gen_rsa() {
        let rsa_pubkey = do_rsa("the seed value".to_string());

        // let key = RsaPublicKey::from_pkcs1_der(RSA_2048_PUB_DER).unwrap();

        // need to convert RsaPublicKey from (n,e) der/pem to JWK
        // JWK.n gets hashed (SHA2-256) then base64URL encoded

        println!("{:?}", rsa_pubkey);

        // assert_eq!(
        //     rsa_pubkey,
        //     hex::decode("74f6b2d13935142e0adf6b275e2fdd6a4bab20f758a2a4fe6369f94312578925")
        //         .unwrap()
        //         .as_slice()
        // );
    }
}
