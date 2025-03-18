use bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> String {
  hash(password, bcrypt::DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
  verify(password, hash).unwrap()
}
