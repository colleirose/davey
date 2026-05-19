use pyo3::prelude::*;
use zeroize::Zeroize;

#[pyclass(get_all)]
#[derive(Clone, Zeroize)]
pub struct SigningKeyPair {
  pub private: Vec<u8>,
  #[zeroize(skip)]
  pub public: Vec<u8>,
}

#[pymethods]
impl SigningKeyPair {
  #[new]
  fn new(private: Vec<u8>, public: Vec<u8>) -> Self {
    Self { private, public }
  }

  fn __del__(&mut self) {
    // In this case, Python is deleting the keypair but it's not necessarily being dropped
    // However, the data in it is no longer needed at this point.
    self.zeroize();
    self.private.clear();
    self.public.clear();
    self.private.shrink_to_fit();
    self.public.shrink_to_fit();
  }

  fn __repr__(&self) -> &'static str {
    let public_key = &self.public;
    let hex = public_key
      .iter()
      .map(|b| format!("{:02x}", b))
      .collect::<String>();
    Box::leak(format!("<SigningKeyPair public={}>", hex).into_boxed_str())
  }
}

impl From<davey::SigningKeyPair> for SigningKeyPair {
  fn from(skp: davey::SigningKeyPair) -> Self {
    SigningKeyPair {
      private: skp.private,
      public: skp.public,
    }
  }
}

#[pyfunction]
pub fn generate_p256_keypair() -> PyResult<SigningKeyPair> {
  let signing_key_pair = davey::SigningKeyPair::generate();
  Ok(SigningKeyPair::from(signing_key_pair))
}
