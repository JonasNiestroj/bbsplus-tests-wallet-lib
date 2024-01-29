use ark_bls12_381::{Bls12_381, Config, FrConfig, G1Affine};
use ark_ec::bls12::Bls12;
use ark_ec::pairing::Pairing;
use ark_ff::{BigInt, Fp, MontBackend, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use ark_std::UniformRand;
use base64::{engine::general_purpose, Engine as _};
use bbs_plus::{
    proof::PoKOfSignatureG1Protocol,
    proof_23::{PoKOfSignature23G1Proof, PoKOfSignature23G1Protocol},
    setup::{
        KeypairG1, KeypairG2, PublicKeyG1, SecretKey, SignatureParams23G1, SignatureParamsG1,
        SignatureParamsG2,
    },
    signature::SignatureG1,
    signature_23::Signature23G1,
};
use blake2::Blake2b512;
use libc::size_t;
use once_cell::sync::{Lazy, OnceCell};
use schnorr_pok::compute_random_oracle_challenge;
use std::collections::{BTreeMap, BTreeSet};

pub type Fr = <Bls12_381 as Pairing>::ScalarField;

static PARAMS: Lazy<OnceCell<SignatureParamsG1<Bls12<Config>>>> = Lazy::new(|| OnceCell::new());

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn create_binding_commitment(params_enc: String) -> String;
        fn create_proof(signature: String) -> String;
    }
}

fn create_binding_commitment(params_enc: String) -> String {
    let params_dec = general_purpose::STANDARD_NO_PAD.decode(params_enc).unwrap();

    let params: SignatureParamsG1<Bls12<Config>> =
        CanonicalDeserialize::deserialize_compressed(&params_dec[..]).unwrap();

    PARAMS.set(params);

    let binding = "asd";

    let mut commited_messages = BTreeMap::new();
    let blind_value = dock_crypto_utils::hashing_utils::field_elem_from_try_and_incr::<
        Fr,
        Blake2b512,
    >(binding.as_bytes());
    commited_messages.insert(0, &blind_value);

    let commitment = PARAMS
        .get()
        .unwrap()
        .commit_to_messages(commited_messages, &blind_value)
        .unwrap();

    let mut ser = vec![];
    CanonicalSerialize::serialize_compressed(&commitment, &mut ser).unwrap();

    let base64 = general_purpose::STANDARD.encode(&ser);

    return base64;
}

fn create_proof(signature: String) -> String {
    let signature_dec = general_purpose::STANDARD.decode(signature).unwrap();
    let messages = vec![
        ("dateOfBirth", "12.07.1999"),
        ("firstName", "Jonas"),
        ("lastName", "Niestroj"),
    ];

    let blind_value = dock_crypto_utils::hashing_utils::field_elem_from_try_and_incr::<
        Fr,
        Blake2b512,
    >("asd".as_bytes());

    let mut enc_messages_temp = BTreeMap::new();

    let mut messages_arr = vec![];

    let mut index = 1;

    messages_arr.push(blind_value);

    for pair in messages {
        let msg = format!("{}: {}", pair.0, pair.1);
        println!("msg {}", msg);
        let enc_msg = dock_crypto_utils::hashing_utils::field_elem_from_try_and_incr::<
            Fr,
            Blake2b512,
        >(msg.as_bytes());

        messages_arr.push(enc_msg);

        enc_messages_temp.insert(index, enc_msg);
        index += 1;
    }
    let sig: SignatureG1<Bls12<Config>> =
        CanonicalDeserialize::deserialize_compressed(&signature_dec[..]).unwrap();
    let unblinded_sig = sig.unblind(&blind_value);

    let mut rng = StdRng::seed_from_u64(0u64);

    let bbs_messages = messages_arr.iter().enumerate().map(|(idx, msg)| {
        if idx == 0 {
            bbs_plus::proof::MessageOrBlinding::BlindMessageRandomly(msg)
        } else {
            bbs_plus::proof::MessageOrBlinding::RevealMessage(msg)
        }
    });

    let mut bbs_messages_temp = vec![];

    for msg in bbs_messages {
        bbs_messages_temp.push(msg);
    }

    let pok = PoKOfSignatureG1Protocol::init(
        &mut rng,
        &unblinded_sig,
        &PARAMS.get().unwrap(),
        bbs_messages_temp,
    )
    .unwrap();

    let committed_indices = vec![0].into_iter().collect::<BTreeSet<usize>>();

    let uncommitted_messages2 = (0..4)
        .filter(|i| !committed_indices.contains(i))
        .map(|i| (i, messages_arr[i]))
        .collect::<BTreeMap<_, _>>();

    let mut chal_bytes_prover = vec![];
    pok.challenge_contribution(
        &uncommitted_messages2,
        PARAMS.get().unwrap(),
        &mut chal_bytes_prover,
    )
    .unwrap();
    let challenger_prover = compute_random_oracle_challenge::<Fr, Blake2b512>(&chal_bytes_prover);

    let proof = pok.gen_proof(&challenger_prover).unwrap();

    let mut ser = vec![];
    CanonicalSerialize::serialize_compressed(&proof, &mut ser).unwrap();

    let base64 = general_purpose::STANDARD.encode(&ser);

    return base64;
}
