use p256::ecdsa::SigningKey;
use rand::rngs::OsRng;
use zeroize::{Zeroize, Zeroizing};

/// A signing key pair. This is needed if you want to pass your own key pair or store the key pair for later.
#[derive(Clone, PartialEq, Eq, Zeroize)]
pub struct SigningKeyPair {
  pub private: Vec<u8>,
  #[zeroize(skip)]
  pub public: Vec<u8>,
}

impl SigningKeyPair {
  /// Generate a signing key pair.
  pub fn generate() -> Self {
    // https://benma.github.io/2020/10/16/rust-zeroize-move.html

    // The signing key pair contains the secret private key value
    let signing_key = Box::new(SigningKey::random(&mut OsRng));
    let private_key_bytes = Box::new(Zeroizing::new(signing_key.to_bytes()));

    let private_key = Box::new(private_key_bytes.to_vec());
    let public_key = signing_key.verifying_key();

    SigningKeyPair {
      private: *private_key,
      public: public_key.to_encoded_point(false).to_bytes().to_vec(),
    }
  }
}
