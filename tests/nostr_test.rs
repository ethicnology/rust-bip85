use bip85_extended::*;
use bitcoin::bech32::{self, Bech32, Hrp};
use bitcoin::bip32::Xpriv;
use bitcoin::hex::DisplayHex;
use bitcoin::secp256k1::Secp256k1;
use std::str::FromStr;

// Official BIP85 Nostr (application 9000') test vectors from
// https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#nostr
const MASTER_XPRV: &str =
    "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb";

struct SpecVector {
    identity: u32,
    account: u32,
    entropy_hex: &'static str,
    nsec: &'static str,
}

const SPEC_VECTORS: &[SpecVector] = &[
    SpecVector {
        identity: 1,
        account: 1,
        entropy_hex: "552ad1d578fe1bc927cec9612651652b07c52dde4017911bc23bc953568075ff",
        nsec: "nsec1254dr4tclcdujf7we9sjv5t99vru2tw7gqtezx7z80y4x45qwhlsmxapst",
    },
    SpecVector {
        identity: 1,
        account: 2,
        entropy_hex: "4fd36c0061a65db375b4350f44bb62a6d7f716ee93bd0f59887ac50b35fa8b96",
        nsec: "nsec1flfkcqrp5ewmxad5x585fwmz5mtlw9hwjw7s7kvg0tzskd063wtq34wlgr",
    },
    SpecVector {
        identity: 2,
        account: 1,
        entropy_hex: "b2d3b48992d46f98beac0196c4e258417087e467dbec1503342785368f4402c2",
        nsec: "nsec1ktfmfzvj63he304vqxtvfcjcg9cg0er8m0kp2qe5y7zndr6yqtpq7q5y44",
    },
];

fn root() -> Xpriv {
    Xpriv::from_str(MASTER_XPRV).unwrap()
}

#[test]
fn test_nostr_spec_vectors() {
    let secp = Secp256k1::new();
    let root = root();
    for v in SPEC_VECTORS {
        let bytes = to_nostr_bytes(&secp, &root, v.identity, v.account).unwrap();
        assert_eq!(
            bytes.to_lower_hex_string(),
            v.entropy_hex,
            "entropy mismatch for identity={}, account={}",
            v.identity,
            v.account,
        );

        let nsec = to_nostr(&secp, &root, v.identity, v.account).unwrap();
        assert_eq!(
            nsec, v.nsec,
            "nsec mismatch for identity={}, account={}",
            v.identity, v.account,
        );
    }
}

#[test]
fn test_nostr_bytes_and_nsec_are_consistent() {
    // The string returned by to_nostr() must be a Bech32 encoding (HRP "nsec")
    // of exactly the 32 bytes returned by to_nostr_bytes().
    let secp = Secp256k1::new();
    let root = root();
    for v in SPEC_VECTORS {
        let bytes = to_nostr_bytes(&secp, &root, v.identity, v.account).unwrap();
        let nsec = to_nostr(&secp, &root, v.identity, v.account).unwrap();

        let (hrp, payload) = bech32::decode(&nsec).unwrap();
        assert_eq!(hrp, Hrp::parse("nsec").unwrap());
        assert_eq!(&payload[..], &bytes[..]);

        // Re-encoding the bytes ourselves must yield the same string.
        let reencoded = bech32::encode::<Bech32>(Hrp::parse("nsec").unwrap(), &bytes).unwrap();
        assert_eq!(reencoded, nsec);
    }
}

#[test]
fn test_nostr_nsec_format() {
    // A NIP19 nsec is always: HRP "nsec" + separator '1' + 52 data chars +
    // 6 checksum chars = 63 characters total, all lower-case.
    let secp = Secp256k1::new();
    let root = root();
    let nsec = to_nostr(&secp, &root, 1, 1).unwrap();
    assert_eq!(nsec.len(), 63);
    assert!(nsec.starts_with("nsec1"));
    assert_eq!(nsec, nsec.to_lowercase());
}

#[test]
fn test_nostr_rejects_non_hardened_indices() {
    let secp = Secp256k1::new();
    let root = root();
    let bad = 0x80000000u32;
    assert_eq!(
        to_nostr(&secp, &root, bad, 1).unwrap_err(),
        Error::InvalidIndex(bad),
    );
    assert_eq!(
        to_nostr(&secp, &root, 1, bad).unwrap_err(),
        Error::InvalidIndex(bad),
    );
    assert_eq!(
        to_nostr_bytes(&secp, &root, bad, 1).unwrap_err(),
        Error::InvalidIndex(bad),
    );
    assert_eq!(
        to_nostr_bytes(&secp, &root, 1, bad).unwrap_err(),
        Error::InvalidIndex(bad),
    );
    // u32::MAX is also out of range.
    assert!(to_nostr(&secp, &root, u32::MAX, 1).is_err());
}

#[test]
fn test_nostr_accepts_boundary_hardened_index() {
    // 0x7FFFFFFF is the largest valid hardened-child input (it derives the
    // m/.../2147483647' child). It must succeed and produce a 63-char nsec.
    let secp = Secp256k1::new();
    let root = root();
    let nsec = to_nostr(&secp, &root, 0x7FFFFFFF, 0x7FFFFFFF).unwrap();
    assert_eq!(nsec.len(), 63);
    assert!(nsec.starts_with("nsec1"));
}

#[test]
fn test_nostr_reserved_zero_indices_still_derive() {
    // The spec marks identity/account 0 as "reserved" but doesn't make them
    // invalid; the reference (bipsea) lets them through. Mirror that contract.
    let secp = Secp256k1::new();
    let root = root();
    let nsec_00 = to_nostr(&secp, &root, 0, 0).unwrap();
    let nsec_01 = to_nostr(&secp, &root, 0, 1).unwrap();
    let nsec_10 = to_nostr(&secp, &root, 1, 0).unwrap();
    let nsec_11 = to_nostr(&secp, &root, 1, 1).unwrap();
    assert!(nsec_00.starts_with("nsec1"));
    // Each (identity, account) pair must produce a distinct key.
    let unique = [&nsec_00, &nsec_01, &nsec_10, &nsec_11];
    for i in 0..unique.len() {
        for j in (i + 1)..unique.len() {
            assert_ne!(unique[i], unique[j]);
        }
    }
}

#[test]
fn test_nostr_distinct_masters_produce_distinct_keys() {
    // Different master xprvs must produce different Nostr keys for the same
    // (identity, account). This catches accidental dropping of the master.
    let secp = Secp256k1::new();
    let a = Xpriv::from_str(MASTER_XPRV).unwrap();
    // A different mainnet xprv, BIP-32 test vector 1 chain m.
    let b = Xpriv::from_str(
        "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi",
    )
    .unwrap();
    assert_ne!(
        to_nostr(&secp, &a, 1, 1).unwrap(),
        to_nostr(&secp, &b, 1, 1).unwrap(),
    );
}
