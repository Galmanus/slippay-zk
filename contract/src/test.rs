#![cfg(test)]
//! Tests for the generic Groth16/BN254 verifier.
//! Fixture is the real proof in ../../build_real/invoke_args.json (the same
//! artifact verify_mainnet.sh runs against the live mainnet contract CBDS2YSL).
//!
//! Coverage:
//!   1. a real proof verifies true,
//!   2. a structurally malformed proof point reverts (off-curve rejected by host),
//!   3. a well-formed but wrong public input verifies false (pairing fails cleanly),
//!   4. a public-input count mismatch verifies false (soundness guard).

use super::*;
use soroban_sdk::{BytesN, Env, Vec};

fn nib(c: u8) -> u8 {
    match c { b'0'..=b'9' => c - b'0', b'a'..=b'f' => c - b'a' + 10,
              b'A'..=b'F' => c - b'A' + 10, _ => 0 }
}
fn bn<const N: usize>(env: &Env, s: &str) -> BytesN<N> {
    let b = s.as_bytes();
    let mut o = [0u8; N];
    let mut i = 0;
    while i < N { o[i] = (nib(b[i * 2]) << 4) | nib(b[i * 2 + 1]); i += 1; }
    BytesN::from_array(env, &o)
}

const ALPHA: &str = "2a8eb39c009e1bb9bb6ac94c30c4900c4affcb499abc453da76d4ce009124f7917207aa813649a28f6d3fdbbb3e8e1d6fd61d41e5a2a0f234781760cbdf994bf";
const BETA: &str  = "002a4124184327a79a117434cd81172a1424a23681332038e0ec9fbfb2dce5671dae45f5a584fc88c8418358b54aa7fb03ea3735ba69c733959c8b8f1c367b4b006fa5ebf1f2e4d1b9b1340dc3d372e638727e2c2210c56cf7d8e618d1dcefa4225d9fa629ed0a84913767236574509a3cb7a294d9aaefe35b1c09d10620a229";
const GAMMA: &str = "198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa";
const DELTA: &str = "028d99d154000e1d3381e6f3f0b668a97d7d56ee70cf7ac9977ff0b8689cdc5a27e37aa0d6b64e466fd127252d46695fe4fb795e6a268faa15476f9512927f980932256b0a99003579269ac5f422cd9d84de73f02bada19f41379741d005cd6b1a0c5650a7576cafee1d49b00716412f5a0e512751d8006e4e9f058202eb988e";
const A: &str = "19efb31e2b9707f4aad960a7fd7200dd533da41b51aa3c5d77d5a4b91b5b8dff010fe3d097bb48e5f931b0b9bf964d8a40a5d84b40f1325c65aa1a7f431d98a4";
const B: &str = "22298f53473ae80c7694487d3234efcce968cd8540aca2e5aeea05ceb136832e15cd4ad3fec880a8f94680fb8fed592f6c9216fed39a7be2099960187ac334dd2fa0d860021e7efa31f08fb319df85062cd674909ea5f509d7287a6e89a2e78b14f034e9e04e1ffbf73a3d1f57a02734c54bfdc278e873544c3f8ded39daad7e";
const C: &str = "2e64bf12b7663e24ab10d515781e273c86b8345b464302be4df83d8ccfd63a7602c52646b70ca9d517fd03017f8e21994dd8ef7b97831247705b0cbfba119c9e";
const IC: [&str; 14] = ["2a5311e2173936c3d299f36663f80d65f917409f5c9e759ba25987f26c775ead05033be74c72c699bfe1c9c858b7b73047658dc6bde1d3f1b64525fd019b1f19", "16395923b2cab93572d41ef7bd9625341c7e7ee847a0a26a71cba529b8d99d0e2302e8096a8525f7953ff02ea01169fd28284c26c5d7721df9f4976010ac166e", "1e248f52fab629a43f1e798eb855fa38620306a3280604b2a926a794e074342824a82ef5115776c84dad6bf495e32f76b524c20a0a1c459d7b85e1136e180b67", "01a37da722388111472ce81364d5909f743174436cc17ee6c96b81ea9f2b3b7d0ec931f3b28139faa9c2691e78c3b28b8491c4bbb8cb4ce97da0ce7cc1fb84b0", "039e7ba5d4dc17ec5c5f46107aa50957feac0f1a8608b4fdda592373b71bc75a1b4224b5cfa4930d014dc450a1b82f93341acb1da1202af32061b1d92bb3f751", "1dda8dc603b88f7affd0f20c46aa6d2edf187a56153cbb0d3fb7795d4e91579e24307e42396d61b7a37e411130ee4d2aa2d5aaba07c9f2a01b0b4b62016c64d4", "0476d26dbe9a77c752f786b726cce5470dbade15087d9b4bde17d600d3ebb0bc1d0c7c662528d194be804e77e54bd2c94d47feb26fc18c422bb91f03ff915ccf", "18a750abff3804b11de5edee0d7c67980a0b45179ce7b0fd682c5bb8c2ff3a4326a2ea7d7c270b69266a7edd6094752761c27c55de9336e0eed52b5e56ee2966", "26b23ec0ffbd5379d0668f090d90354d94bbc06c06bc3dc5e2a23391d03218d9221400800fec262d30ba16fc4e98a3568c59fbec7129ee0f836b6c4d39c69f29", "2344dba11a7ff154fb3f7f420715efbe688d7839db4920062470a51c2d3b29562e7d3984af22f0006b8314e89c253de53ad25507146dac3921adba1d9c0a3389", "1570f178febaf7ef42195abeae2c8dff1490f327c26c279105011f3a1626cbee2914d1b1bde5d3a20ddeb14bdb442c4be373f8dd94fc6ce35aebaea8dd2c79da", "05ab97e7d827ffb6181574252fd2bdf35061810557bc549fe893e64bfcada5c32f738c9390ee9afc3397bd3ccca18fada46d712b84fe3e845cbb7cefbb1929d8", "24563312d775f6579f29c9afd4ad908a0a00a17fc1cc40b1a252b01dc58bbf6f2542b5bf62bb117b05815378be21e08b9ce56b80cafcdd27c978dbad20332198", "09c8945ed83b1b40bc542d3603498289ffc1ae0f56edad80e5410549f01005e423875130319da913c406c910d70b31391f94618f060a1b5915811c794199501e"];
const PUBS: [&str; 13] = ["0000000000000000000000000000000000000000000000000000000000000001", "1bb1e68cae1c9aaa37ab5cd32a6bef0ceee1fff56c540e83dbeb94796b0f8748", "2e7a864cff377667539d6230976b0c96eaa885132468c373574b8726d5593ec0", "05763339b16e7e305e0c72f14c3deedf35258fd36f496b195428a852da1004d0", "17f1c65bdc6ef31a726189f80bd8214939e4c3e6a3364ca556d48dda70242c81", "00000000000000000000000000000000000000000000000000000000000007d0", "0000000000000000000000000000000000000000000000000000000000000fa0", "0888022f757bfe7a48078656d61b10133bf6d2d0930e66ccde5a9681ac860b01", "2a4634ac64807549525388a980df7aa307fed88e5eb0a108f61a9cf5cdf60fa9", "0888022f757bfe7a48078656d61b10133bf6d2d0930e66ccde5a9681ac860b01", "0888022f757bfe7a48078656d61b10133bf6d2d0930e66ccde5a9681ac860b01", "085ed469c9a9f102b6d4f6f909b8ceaf6ca49b39759ac2e0feb7e0aada8b7111", "245e25ab2bd42f0280a5ade750828dd6868f5225ae798d6b51c676f519c8f4e8"];

fn ic_vec(env: &Env) -> Vec<BytesN<64>> {
    let mut v = Vec::new(env);
    for h in IC.iter() { v.push_back(bn::<64>(env, h)); }
    v
}
fn pubs_vec(env: &Env) -> Vec<BytesN<32>> {
    let mut v = Vec::new(env);
    for h in PUBS.iter() { v.push_back(bn::<32>(env, h)); }
    v
}
fn client(env: &Env) -> SlippayZkVerifierClient<'_> {
    SlippayZkVerifierClient::new(env, &env.register(SlippayZkVerifier, ()))
}

#[test]
fn verify_accepts_valid_proof() {
    let env = Env::default();
    let ok = client(&env).verify(
        &bn::<64>(&env, ALPHA), &bn::<128>(&env, BETA), &bn::<128>(&env, GAMMA),
        &bn::<128>(&env, DELTA), &ic_vec(&env), &bn::<64>(&env, A),
        &bn::<128>(&env, B), &bn::<64>(&env, C), &pubs_vec(&env),
    );
    assert!(ok, "real proof must verify true");
}

#[test]
fn verify_reverts_on_malformed_proof_point() {
    // Flip one byte of A's X coordinate -> point off the BN254 curve.
    // The host rejects it (Crypto/InvalidInput), so the invocation reverts.
    // try_verify captures that as Err instead of panicking the test.
    let env = Env::default();
    let mut a = bn::<64>(&env, A).to_array();
    a[0] ^= 0x01;
    let bad_a = BytesN::from_array(&env, &a);
    let res = client(&env).try_verify(
        &bn::<64>(&env, ALPHA), &bn::<128>(&env, BETA), &bn::<128>(&env, GAMMA),
        &bn::<128>(&env, DELTA), &ic_vec(&env), &bad_a,
        &bn::<128>(&env, B), &bn::<64>(&env, C), &pubs_vec(&env),
    );
    assert!(res.is_err(), "malformed (off-curve) proof point must revert");
}

#[test]
fn verify_rejects_wrong_public_input() {
    // Replace a public input (index 1) with a different, well-formed small scalar.
    // The point math stays valid, but vk_x changes, so pairing_check returns false.
    let env = Env::default();
    let mut pv = pubs_vec(&env);
    let mut s = [0u8; 32];
    s[31] = 0x02; // valid Fr, different from the real input
    pv.set(1, BytesN::from_array(&env, &s));
    let ok = client(&env).verify(
        &bn::<64>(&env, ALPHA), &bn::<128>(&env, BETA), &bn::<128>(&env, GAMMA),
        &bn::<128>(&env, DELTA), &ic_vec(&env), &bn::<64>(&env, A),
        &bn::<128>(&env, B), &bn::<64>(&env, C), &pv,
    );
    assert!(!ok, "wrong public input must verify false");
}

#[test]
fn verify_rejects_public_input_count_mismatch() {
    // SOUNDNESS guard: ic.len() must equal pubs.len()+1, so a prover cannot omit
    // a public input (e.g. the regulator key) and still pass. Drop one IC point.
    let env = Env::default();
    let mut ic = ic_vec(&env);
    ic.pop_back();
    let ok = client(&env).verify(
        &bn::<64>(&env, ALPHA), &bn::<128>(&env, BETA), &bn::<128>(&env, GAMMA),
        &bn::<128>(&env, DELTA), &ic, &bn::<64>(&env, A),
        &bn::<128>(&env, B), &bn::<64>(&env, C), &pubs_vec(&env),
    );
    assert!(!ok, "public-input count mismatch must verify false");
}
