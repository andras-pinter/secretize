use crate::Secret;

impl PartialEq for Secret {
    fn eq(&self, other: &Self) -> bool {
        self.secret.eq(&other.secret)
    }
}

#[cfg(feature = "eq")]
impl<S: AsRef<[u8]>> PartialEq<S> for Secret {
    fn eq(&self, other: &S) -> bool {
        self.verify(other)
    }
}
