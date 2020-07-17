use crate::Error;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use serde_hex::{encode as hex_encode, PrefixedHexVisitor};
use ssz::{Decode, Encode};
use std::fmt;
use std::hash::{Hash, Hasher};
use tree_hash::TreeHash;

/// The byte-length of a BLS public key when serialized in compressed form.
pub const PUBLIC_KEY_BYTES_LEN: usize = 48;

/// Implemented on some struct from a BLS library so it may be used as the `point` in a
/// `GenericPublicKey`.
pub trait TPublicKey: Sized + Clone {
    /// Serialize `self` as compressed bytes.
    fn serialize(&self) -> [u8; PUBLIC_KEY_BYTES_LEN];

    /// Deserialize `self` from compressed bytes.
    fn deserialize(bytes: &[u8]) -> Result<Self, Error>;
}

/// A BLS aggregate public key that is generic across some BLS point (`Pub`).
///
/// Provides generic functionality whilst deferring all serious cryptographic operations to `Pub`.
#[derive(Clone)]
pub struct GenericPublicKey<Pub> {
    /// The underlying point which performs *actual* cryptographic operations.
    point: Pub,
}

impl<Pub> GenericPublicKey<Pub>
where
    Pub: TPublicKey,
{
    /// Instantiates `Self` from a `point`.
    pub(crate) fn from_point(point: Pub) -> Self {
        Self { point }
    }

    /// Returns a reference to the underlying BLS point.
    pub(crate) fn point(&self) -> &Pub {
        &self.point
    }

    /// Returns `self.serialize()` as a `0x`-prefixed hex string.
    pub fn to_hex_string(&self) -> String {
        format!("{:?}", self)
    }

    /// Serialize `self` as compressed bytes.
    pub fn serialize(&self) -> [u8; PUBLIC_KEY_BYTES_LEN] {
        self.point.serialize()
    }

    /// Deserialize `self` from compressed bytes.
    pub fn deserialize(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            point: Pub::deserialize(bytes)?,
        })
    }
}

impl<Pub: TPublicKey + Eq> Eq for GenericPublicKey<Pub> {}

impl<Pub: TPublicKey> PartialEq for GenericPublicKey<Pub> {
    fn eq(&self, other: &Self) -> bool {
        &self.serialize()[..] == &other.serialize()[..]
    }
}

/// Hashes the `self.serialize()` bytes.
impl<Pub: TPublicKey> Hash for GenericPublicKey<Pub> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.serialize()[..].hash(state);
    }
}

impl<Pub: TPublicKey> Encode for GenericPublicKey<Pub> {
    impl_ssz_encode!(PUBLIC_KEY_BYTES_LEN);
}

impl<Pub: TPublicKey> Decode for GenericPublicKey<Pub> {
    impl_ssz_decode!(PUBLIC_KEY_BYTES_LEN);
}

impl<Pub: TPublicKey> TreeHash for GenericPublicKey<Pub> {
    impl_tree_hash!(PUBLIC_KEY_BYTES_LEN);
}

impl<Pub: TPublicKey> Serialize for GenericPublicKey<Pub> {
    impl_serde_serialize!();
}

impl<'de, Pub: TPublicKey> Deserialize<'de> for GenericPublicKey<Pub> {
    impl_serde_deserialize!();
}

impl<Pub: TPublicKey> fmt::Debug for GenericPublicKey<Pub> {
    impl_debug!();
}

#[cfg(feature = "arbitrary")]
impl<Pub: TPublicKey + 'static> arbitrary::Arbitrary for GenericPublicKey<Pub> {
    impl_arbitrary!(PUBLIC_KEY_BYTES_LEN);
}
