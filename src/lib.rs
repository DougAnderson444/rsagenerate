use hmac_drbg::*;
use rand::{rngs::StdRng, SeedableRng};
// in order to use a Rust Trait, you need to 'use' it
use base64;
use der::Decodable;
use rsa::{
    pkcs1::{
        EncodeRsaPrivateKey, EncodeRsaPublicKey, RsaPrivateKey as Pkcs1RsaPrivateKey,
        RsaPrivateKeyDocument, RsaPublicKey as Pkcs1RsaPublicKey, RsaPublicKeyDocument,
    },
    pkcs8::{EncodePublicKey, PublicKeyDocument},
    Hash, PaddingScheme, RsaPrivateKey, RsaPublicKey,
};
use sha2::{Digest, Sha256};
use typenum::consts::*;

pub struct Keypair {
    priv_key: RsaPrivateKey,
    pub_key: RsaPublicKey,
}

pub fn get_address(seed: String) -> String {
    let kp = do_rsa(seed);
    let b64url = pub_der_to_jwk(&kp.pub_key);
    b64url
}

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

pub fn do_rsa(seed: String) -> Keypair {
    let mut rng = StdRng::from_seed(determine_bytes(seed));
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    Keypair { priv_key, pub_key }
}

fn priv_der_to_jwk(priv_key: &RsaPrivateKey) {
    let priv_der: RsaPrivateKeyDocument = priv_key.to_pkcs1_der().unwrap();
    let converted = Pkcs1RsaPrivateKey::from_der(priv_der.as_ref()).unwrap();

    let digest = Sha256::digest(converted.modulus.as_bytes());
    let b64_url = base64::encode_config(&digest, base64::URL_SAFE);
    println!("Base64_URL hashed modulus bytes: \n {:?}", b64_url);
}

fn pub_der_to_jwk(pub_key: &RsaPublicKey) -> String {
    let pub_der: RsaPublicKeyDocument = pub_key.to_pkcs1_der().unwrap();
    let converted = Pkcs1RsaPublicKey::from_der(pub_der.as_ref()).unwrap();

    // println!("Base64_URL hashed modulus: \n {:?}", converted.modulus);

    let digest = Sha256::digest(converted.modulus.as_bytes()); // no specified length
    let b64_url = base64::encode_config(&digest, base64::URL_SAFE);
    println!("Base64_URL hashed modulus bytes: \n {:?}", b64_url);

    let pub_der: PublicKeyDocument = pub_key.to_public_key_der().unwrap();

    b64_url
}

pub fn sign() {
    // let private_key = RSAPrivateKey::from_pkcs1(&der_bytes).unwrap();
    // let result = private_key.sign(PaddingScheme::PKCS1v15Sign{hash:Option::from(Hash::SHA2_256)},  &hasher(source)).unwrap();
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

        // println!("{:?}", rsa_pubkey);

        // assert_eq!(
        //     rsa_pubkey,
        //     "de7_-o7hKWqv_NDBZneDfZH6MP8aF2uw504UT9gXeEE=".to_string()
        // );
    }
}
