// Copyright (c) 2018-2021 MobileCoin Inc.

//! Utilities for handling X509 certificate chains

use displaydoc::Display;
use mc_crypto_keys::{DistinguishedEncoding, Ed25519Public, KeyError};
use pem::Pem;
use std::{
    convert::TryFrom,
    time::{SystemTime, SystemTimeError},
};
use x509_signature::{ASN1Time, Error as X509Error, X509Certificate};

/// An iterator of [`X509Certificate`] objects over a slice of [`Pem`] objects.
pub struct X509CertificateIter<'a> {
    pem_slice: &'a [Pem],
    offset: usize,
}

impl<'a> X509CertificateIter<'a> {
    fn new<T: AsRef<[Pem]>>(pems: &'a T) -> Self {
        Self {
            pem_slice: pems.as_ref(),
            offset: 0,
        }
    }
}

impl<'a> Iterator for X509CertificateIter<'a> {
    type Item = X509Certificate<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == self.pem_slice.len() {
            None
        } else {
            self.offset += 1;
            x509_signature::parse_certificate(&self.pem_slice[self.offset - 1].contents).ok()
        }
    }
}

/// A trait used to monkey-patch an X509Certificate parsing iterator over a
/// slice of [`Pem`] objects.
pub trait X509CertificateIterable {
    fn iter_x509(&self) -> X509CertificateIter;
}

/// Anything which can be referenced as a Pem slice gets a method to iterate
/// certificates, and verify it is a well-formed single-path certificate chain.
impl<T: AsRef<[Pem]>> X509CertificateIterable for T {
    fn iter_x509(&self) -> X509CertificateIter {
        X509CertificateIter::new(self)
    }
}

/// An emumeration of errors
#[derive(Debug, Display, Eq, PartialEq)]
pub enum ChainError {
    /// The chain slice is empty
    Empty,
    /// Could not retrieve the current time: second time provided was later than
    /// self
    SystemTime,
    /// X509 error: {0:?}
    X509(X509Error),
}

impl From<SystemTimeError> for ChainError {
    fn from(_src: SystemTimeError) -> ChainError {
        ChainError::SystemTime
    }
}

impl From<X509Error> for ChainError {
    fn from(src: X509Error) -> ChainError {
        ChainError::X509(src)
    }
}

/// A trait used to monkey-patch an X509Certificate chain verifier onto a slice
/// of X509Certificate objects.
pub trait X509CertificateChain {
    /// Verify the chain (checks validity, signatures, and CA extension of each
    /// element)
    fn verify_chain(&self) -> Result<usize, ChainError>;
}

impl<'a, T: AsRef<[X509Certificate<'a>]>> X509CertificateChain for T {
    fn verify_chain(&self) -> Result<usize, ChainError> {
        let mut previous = None;
        let mut cert_count = 0usize;

        if self.as_ref().is_empty() {
            return Err(ChainError::Empty);
        }

        for (index, cert) in self.as_ref().iter().enumerate() {
            // If the cert wasn't signed by the preceeding cert (or itself, if first), fail.
            if let Some(prev_cert) = previous {
                cert.check_issued_by(prev_cert)?;
            } else {
                cert.check_self_issued()?;
            }

            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs() as i64;
            eprintln!(
                "Cert: Now: {:?}, Not Before: {:?}, Not After: {:?}",
                ASN1Time::try_from(timestamp).expect("asn1 timestamp of now is incorrect"),
                cert.not_before(),
                cert.not_after()
            );

            // If the cert isn't valid (temporally), fail.
            cert.valid_at_timestamp(timestamp)?;

            // Update state for the next iteration
            previous = Some(cert);
            // Update the number of certificates which have passed validation
            cert_count = index + 1;
        }

        Ok(cert_count)
    }
}

/// A list of key types supported by both X.509 and mc-crypto-keys.
pub enum PublicKeyType {
    /// The public key is Ed25519
    Ed25519(Ed25519Public),
}

/// Monkey-patch support for extracting mc-crypto-keys public keys from X509
/// certificates.
pub trait X509KeyExtrator {
    /// Try to retrieve the public key.
    fn mc_public_key(&self) -> Result<PublicKeyType, KeyError>;
}

impl X509KeyExtrator for X509Certificate<'_> {
    fn mc_public_key(&self) -> Result<PublicKeyType, KeyError> {
        if let Ok(pubkey) = Ed25519Public::try_from_der(self.subject_public_key_info().spki()) {
            Ok(PublicKeyType::Ed25519(pubkey))
        } else {
            Err(KeyError::AlgorithmMismatch)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mc_crypto_x509_test_vectors as test_vectors;

    const TYPICAL_CHAIN_LEN: usize = 3;
    const DEPTH10_CHAIN_LEN: usize = 10;

    /// Ensure a valid RSA-RSA-Ed25519 chain is validated correctly.
    #[test]
    fn valid_chain() {
        let (pem_string, pair) = test_vectors::ok_rsa_chain_25519_leaf();

        let cert_ders = pem::parse_many(pem_string);
        let certs = cert_ders.iter_x509().collect::<Vec<X509Certificate>>();

        assert_eq!(
            TYPICAL_CHAIN_LEN,
            certs.verify_chain().expect("Could not verify valid chain")
        );

        let pubkey = certs
            .last()
            .expect("Could not get last cert")
            .mc_public_key()
            .expect("Could not parse last cert's pubkey");

        match pubkey {
            PublicKeyType::Ed25519(key) => assert_eq!(pair.public_key(), key),
        }
    }

    /// Ensure a longer (but still valid) chain is validated correctly.
    #[test]
    fn depth10_chain() {
        let (pem_string, pair) = test_vectors::ok_rsa_chain_depth_10();

        let cert_ders = pem::parse_many(pem_string);
        let certs = cert_ders.iter_x509().collect::<Vec<X509Certificate>>();

        assert_eq!(
            DEPTH10_CHAIN_LEN,
            certs.verify_chain().expect("Could not verify valid chain")
        );

        let pubkey = certs
            .last()
            .expect("Could not get last cert")
            .mc_public_key()
            .expect("Could not parse last cert's pubkey");

        match pubkey {
            PublicKeyType::Ed25519(key) => assert_eq!(pair.public_key(), key),
        }
    }

    /// Ensure a (valid) tree of certs is not verified as a chain.
    #[test]
    fn tree_not_chain() {
        let pem_string = test_vectors::ok_rsa_tree();

        let cert_ders = pem::parse_many(pem_string);
        let certs = cert_ders.iter_x509().collect::<Vec<X509Certificate>>();
        let err = certs
            .verify_chain()
            .expect_err("Verification of tree-not-chain did not fail");

        assert_eq!(err, ChainError::X509(X509Error::UnknownIssuer));
    }

    /// Ensure a certificate chain missing its root (no root, intermediate,
    /// leaf) is not verified.
    #[test]
    fn missing_head() {
        let (pem_string, _pair) = test_vectors::fail_missing_head();

        let cert_ders = pem::parse_many(pem_string);
        let certs = cert_ders.iter_x509().collect::<Vec<X509Certificate>>();
        let err = certs
            .verify_chain()
            .expect_err("Verification of missing head chain did not fail");

        assert_eq!(err, ChainError::X509(X509Error::UnknownIssuer));
    }

    /// Ensure a broken chain (root, no intermediate, leaf) is not verified.
    #[test]
    fn missing_link() {
        let (pem_string, _pair) = test_vectors::fail_missing_link();

        let cert_ders = pem::parse_many(pem_string);
        let certs = cert_ders.iter_x509().collect::<Vec<X509Certificate>>();
        let err = certs
            .verify_chain()
            .expect_err("Verification of missing-link chain did not fail");

        assert_eq!(err, ChainError::X509(X509Error::UnknownIssuer));
    }

    /// Ensure a chain containing a not-yet-valid cert is not verified.
    #[test]
    fn too_soon() {
        let (pem_string, _pair) = test_vectors::fail_leaf_too_soon();

        let cert_ders = pem::parse_many(pem_string);
        let certs = cert_ders.iter_x509().collect::<Vec<X509Certificate>>();
        let err = certs
            .verify_chain()
            .expect_err("Verification of not-yet-valid chain did not fail");

        // FIXME: Investigate
        assert_eq!(err, ChainError::X509(X509Error::CertNotValidYet));
    }

    /// Ensure a chain containing an expired cert is not verified.
    #[test]
    fn expired() {
        let (pem_string, _pair) = test_vectors::fail_leaf_expired();

        let cert_ders = pem::parse_many(pem_string);
        let certs = cert_ders.iter_x509().collect::<Vec<X509Certificate>>();
        let err = certs
            .verify_chain()
            .expect_err("Verification of expired chain did not fail");

        assert_eq!(err, ChainError::X509(X509Error::CertExpired));
    }
}
