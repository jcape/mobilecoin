#![no_std]

//! Simplified API for using Schnorrkel in a deterministic manner, with simple
//! ristretto key pairs, where the public key is a RistretoPoint and the private key is a Scalar.
//!
//! mc-crypto-keys crate provides wrappers RistrettoPublic and RistrettoPrivate around these
//! and implements many handy traits for performing high-level cryptography operations,
//! and this crate provides a way to create signatures that is compatible with these key pairs.

use core::convert::TryInto;
use mc_crypto_keys::{RistrettoPrivate, RistrettoPublic};
pub use schnorrkel::{Signature, SignatureError, SIGNATURE_LENGTH};

/// Create a deterministic Schnorrkel signature
///
/// Arguments:
/// * context_tag: Domain separation tag for the signatures
/// * private_key: The RistrettoPrivate key used to sign the message
/// * message: The message that is signed
///
/// Returns:
/// * A 64-byte Schnorrkel Signature object which can be converted to and from bytes using its API.
pub fn sign(
    context_tag: &'static [u8],
    private_key: &RistrettoPrivate,
    message: &[u8],
) -> Signature {
    private_key.sign_schnorrkel(context_tag, message).try_into().expect("Could not convert RistrettoSignature made from schnorrkel::Signature back to schnorrkel::Signature")
}

/// Verify a Schnorrkel signature
///
/// Note that this should work correctly even with Schnorrkel signatures not generated by the sign function
/// above, because the details of the nonce generation don't affect whether the signature passes verification.
/// The signing context bytes will matter though, if the other party is using a special signing context then
/// we must provide the same signing context bytes.
///
/// Arguments:
/// * context_tag: Domain separation tag for the signatures.
/// * public_key: Public key to check the signature against.
/// * message: The message that is signed.
/// * signature: The signature object to verify.
///
/// Returns:
/// * Ok if the signature checks out, SignatureError otherwise.
pub fn verify(
    context_tag: &'static [u8],
    public_key: &RistrettoPublic,
    message: &[u8],
    signature: &Signature,
) -> Result<(), SignatureError> {
    public_key.verify_schnorrkel(context_tag, message, &signature.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mc_util_from_random::FromRandom;
    use mc_util_test_helper::run_with_several_seeds;

    mod compat_20210122 {
        use crate::*;
        use core::convert::TryFrom;

        // These example files were created by hacking the unit tests in
        // the tests module to save their data to /tmp, and then manually
        // importing it into the repo. The unit tests above use
        // run_with_several_seeds() to execute multiple iterations of the
        // test with different randomized inputs, and the outputs were
        // saved to directories of the pattern:
        //
        // /tmp/testdata/<function>/<iteration>/data.bin
        //
        // It's useful to use .bin here because gitattributes specifies
        // files with that extension be treated as binary.

        const EXPECTED_SUCCESS_SECKEY: [&[u8; 32]; 3] = [
            include_bytes!("testdata/20210121/expected_success/0/seckey.bin"),
            include_bytes!("testdata/20210121/expected_success/1/seckey.bin"),
            include_bytes!("testdata/20210121/expected_success/2/seckey.bin"),
        ];

        const EXPECTED_SUCCESS_PUBKEY: [&[u8; 32]; 3] = [
            include_bytes!("testdata/20210121/expected_success/0/pubkey.bin"),
            include_bytes!("testdata/20210121/expected_success/1/pubkey.bin"),
            include_bytes!("testdata/20210121/expected_success/2/pubkey.bin"),
        ];

        const EXPECTED_SUCCESS_SIG: [&[u8; 64]; 3] = [
            include_bytes!("testdata/20210121/expected_success/0/sig.bin"),
            include_bytes!("testdata/20210121/expected_success/1/sig.bin"),
            include_bytes!("testdata/20210121/expected_success/2/sig.bin"),
        ];

        #[test]
        fn expected_success() {
            for i in 0..2 {
                let seckey = RistrettoPrivate::try_from(EXPECTED_SUCCESS_SECKEY[i])
                    .expect("Could not load seckey");
                let pubkey = RistrettoPublic::try_from(EXPECTED_SUCCESS_PUBKEY[i])
                    .expect("Could not load pubkey");
                let sig = Signature::from_bytes(EXPECTED_SUCCESS_SIG[i])
                    .expect("Could not load signature");

                verify(b"test", &pubkey, b"foobar", &sig).expect("unexpected failure");

                let sig2 = sign(b"test", &seckey, b"foobar");
                assert_eq!(sig2, sig);
            }
        }

        const EXPECTED_FAILURE_BAD_CONTEXT_SECKEY: [&[u8; 32]; 3] = [
            include_bytes!("testdata/20210121/expected_failure_bad_context/0/seckey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_context/1/seckey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_context/2/seckey.bin"),
        ];
        const EXPECTED_FAILURE_BAD_CONTEXT_PUBKEY: [&[u8; 32]; 3] = [
            include_bytes!("testdata/20210121/expected_failure_bad_context/0/pubkey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_context/1/pubkey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_context/2/pubkey.bin"),
        ];
        const EXPECTED_FAILURE_BAD_CONTEXT_SIG: [&[u8; 64]; 3] = [
            include_bytes!("testdata/20210121/expected_failure_bad_context/0/sig.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_context/1/sig.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_context/2/sig.bin"),
        ];

        #[test]
        fn expected_failure_bad_context() {
            for i in 0..2 {
                let seckey = RistrettoPrivate::try_from(EXPECTED_FAILURE_BAD_CONTEXT_SECKEY[i])
                    .expect("Could not load seckey");
                let pubkey = RistrettoPublic::try_from(EXPECTED_FAILURE_BAD_CONTEXT_PUBKEY[i])
                    .expect("Could not load pubkey");
                let sig = Signature::from_bytes(EXPECTED_FAILURE_BAD_CONTEXT_SIG[i])
                    .expect("Could not load signature");

                assert!(verify(b"prod", &pubkey, b"foobar", &sig).is_err());

                let sig2 = sign(b"test", &seckey, b"foobar");
                assert_eq!(sig2, sig);
            }
        }

        const EXPECTED_FAILURE_BAD_KEYS_SECKEY: [&[u8; 32]; 3] = [
            include_bytes!("testdata/20210121/expected_failure_bad_keys/0/seckey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_keys/1/seckey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_keys/2/seckey.bin"),
        ];
        const EXPECTED_FAILURE_BAD_KEYS_SECKEY2: [&[u8; 32]; 3] = [
            include_bytes!("testdata/20210121/expected_failure_bad_keys/0/seckey2.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_keys/1/seckey2.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_keys/2/seckey2.bin"),
        ];
        const EXPECTED_FAILURE_BAD_KEYS_PUBKEY: [&[u8; 32]; 3] = [
            include_bytes!("testdata/20210121/expected_failure_bad_keys/0/pubkey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_keys/1/pubkey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_keys/2/pubkey.bin"),
        ];
        const EXPECTED_FAILURE_BAD_KEYS_SIG: [&[u8; 64]; 3] = [
            include_bytes!("testdata/20210121/expected_failure_bad_keys/0/sig.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_keys/1/sig.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_keys/2/sig.bin"),
        ];

        #[test]
        fn expected_failure_bad_keys() {
            for i in 0..2 {
                let seckey = RistrettoPrivate::try_from(EXPECTED_FAILURE_BAD_KEYS_SECKEY[i])
                    .expect("Could not load seckey");
                let seckey2 = RistrettoPrivate::try_from(EXPECTED_FAILURE_BAD_KEYS_SECKEY2[i])
                    .expect("Could not load seckey");
                let pubkey = RistrettoPublic::try_from(EXPECTED_FAILURE_BAD_KEYS_PUBKEY[i])
                    .expect("Could not load pubkey");
                let sig = Signature::from_bytes(EXPECTED_FAILURE_BAD_KEYS_SIG[i])
                    .expect("Could not load signature");

                assert!(verify(b"test", &pubkey, b"foobar", &sig).is_err());

                let sig1 = sign(b"test", &seckey, b"foobar");
                let sig2 = sign(b"test", &seckey2, b"foobar");
                assert_ne!(sig1, sig2);
                assert_eq!(sig2, sig);
            }
        }

        const EXPECTED_FAILURE_BAD_MESSAGE_SECKEY: [&[u8; 32]; 3] = [
            include_bytes!("testdata/20210121/expected_failure_bad_message/0/seckey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_message/1/seckey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_message/2/seckey.bin"),
        ];
        const EXPECTED_FAILURE_BAD_MESSAGE_PUBKEY: [&[u8; 32]; 3] = [
            include_bytes!("testdata/20210121/expected_failure_bad_message/0/pubkey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_message/1/pubkey.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_message/2/pubkey.bin"),
        ];
        const EXPECTED_FAILURE_BAD_MESSAGE_SIG: [&[u8; 64]; 3] = [
            include_bytes!("testdata/20210121/expected_failure_bad_message/0/sig.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_message/1/sig.bin"),
            include_bytes!("testdata/20210121/expected_failure_bad_message/2/sig.bin"),
        ];

        #[test]
        fn expected_failure_bad_message() {
            for i in 0..2 {
                let seckey = RistrettoPrivate::try_from(EXPECTED_FAILURE_BAD_MESSAGE_SECKEY[i])
                    .expect("Could not load seckey");
                let pubkey = RistrettoPublic::try_from(EXPECTED_FAILURE_BAD_MESSAGE_PUBKEY[i])
                    .expect("Could not load pubkey");
                let sig = Signature::from_bytes(EXPECTED_FAILURE_BAD_MESSAGE_SIG[i])
                    .expect("Could not load signature");

                assert!(verify(b"test", &pubkey, b"foobarbaz", &sig).is_err());

                let sig2 = sign(b"test", &seckey, b"foobar");
                assert_eq!(sig2, sig);
            }
        }
    }

    // Expected successes
    #[test]
    fn expected_success() {
        run_with_several_seeds(|mut rng| {
            let seckey = RistrettoPrivate::from_random(&mut rng);
            let pubkey = RistrettoPublic::from(&seckey);

            let sig = sign(b"test", &seckey, b"foobar");
            verify(b"test", &pubkey, b"foobar", &sig).expect("unexpected failure");
        })
    }
    // Expected failure when key is different
    #[test]
    fn expected_failure_bad_keys() {
        run_with_several_seeds(|mut rng| {
            let seckey = RistrettoPrivate::from_random(&mut rng);
            let seckey2 = RistrettoPrivate::from_random(&mut rng);
            let pubkey = RistrettoPublic::from(&seckey);

            let sig = sign(b"test", &seckey2, b"foobar");
            let result = verify(b"test", &pubkey, b"foobar", &sig);
            assert!(!result.is_ok());
        })
    }
    // Expected failure when message is different
    #[test]
    fn expected_failure_bad_message() {
        run_with_several_seeds(|mut rng| {
            let seckey = RistrettoPrivate::from_random(&mut rng);
            let pubkey = RistrettoPublic::from(&seckey);

            let sig = sign(b"test", &seckey, b"foobar");
            let result = verify(b"test", &pubkey, b"foobarbaz", &sig);
            assert!(!result.is_ok());
        })
    }

    // Expected failure when context is different
    #[test]
    fn expected_failure_bad_context() {
        run_with_several_seeds(|mut rng| {
            let seckey = RistrettoPrivate::from_random(&mut rng);
            let pubkey = RistrettoPublic::from(&seckey);

            let sig = sign(b"test", &seckey, b"foobar");
            let result = verify(b"prod", &pubkey, b"foobar", &sig);
            assert!(!result.is_ok());
        })
    }
}
