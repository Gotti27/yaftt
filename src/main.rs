mod ble_auth;

use p256::{EncodedPoint, ecdh::EphemeralSecret};
use rand_core::OsRng; // requires 'getrandom' feature

fn main() {


    // Alice
    let alice_sk = EphemeralSecret::random(&mut OsRng);
    let alice_pk = alice_sk.public_key();
    let alice_pk_bytes = EncodedPoint::from(alice_pk);

    // Bob
    let bob_sk = EphemeralSecret::random(&mut OsRng);
    let bob_pk = bob_sk.public_key();
    let bob_pk_bytes = EncodedPoint::from(bob_pk);

    let alice_shared = alice_sk.diffie_hellman(&bob_pk);

    let bob_shared = bob_sk.diffie_hellman(&alice_pk);

    // Use the shared secret for further cryptographic operations
    println!("Alice Shared: {:?}", alice_shared.raw_secret_bytes());
    println!("Bob Shared:   {:?}", bob_shared.raw_secret_bytes());
}
