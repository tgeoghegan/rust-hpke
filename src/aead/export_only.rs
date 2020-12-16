use crate::aead::Aead;

use aead::{AeadInPlace as BaseAead, NewAead as BaseNewAead};
use generic_array::typenum;

/// An inert underlying Aead implementation. The open/seal routines panic. The `new()` function
/// returns an `EmptyAeadImpl`, and that is all of the functionality this struct has.
#[doc(hidden)]
#[derive(Clone)]
pub struct EmptyAeadImpl;

impl BaseAead for EmptyAeadImpl {
    // The nonce size has to be bigger than the sequence size (currently u64), otherwise we get an
    // underflow error on seal()/open() before we can even panic
    type NonceSize = typenum::U128;
    type TagSize = typenum::U0;
    type CiphertextOverhead = typenum::U0;

    fn encrypt_in_place_detached(
        &self,
        _: &aead::Nonce<Self::NonceSize>,
        _: &[u8],
        _: &mut [u8],
    ) -> Result<aead::Tag<Self::TagSize>, aead::Error> {
        panic!("Cannot encrypt with an empty encryption context!");
    }

    fn decrypt_in_place_detached(
        &self,
        _: &aead::Nonce<Self::NonceSize>,
        _: &[u8],
        _: &mut [u8],
        _: &aead::Tag<Self::TagSize>,
    ) -> Result<(), aead::Error> {
        panic!("Cannot decrypt with an empty encryption context!");
    }
}

impl BaseNewAead for EmptyAeadImpl {
    type KeySize = typenum::U0;

    // Ignore the key, just return the object
    fn new(_: &aead::Key<Self>) -> Self {
        EmptyAeadImpl
    }
}

/// An AEAD which can **only** be used for its `export()` function. The `open()` and `seal()`
/// methods on an `AeadCtxR` or `AeadCtxS` which uses this AEAD underlyingly **will panic** if you
/// call them
pub struct ExportOnlyAead;

impl Aead for ExportOnlyAead {
    type AeadImpl = EmptyAeadImpl;

    // draft07 §7.3: Export-only
    const AEAD_ID: u16 = 0xFFFF;
}
