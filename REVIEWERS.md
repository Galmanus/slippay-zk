# Reviewers: verify in 60 seconds

This repo ships **confidential compliance for Stellar payments**: zero-knowledge
proofs of KYC and of agent-payment mandates that verify on a generic Groth16/BN254
verifier **live on Stellar mainnet** (Protocol 26, the X-Ray BN254 host functions).
Drop-in for any Stellar anchor or wallet that needs to prove a user is compliant
without putting PII or amounts on a public ledger.

## The 60-second check

1. **The verifier is live on mainnet.** Open it on the explorer:
   - contract `CBDS2YSLATINQVUDG5Y5HV4KQBEAVFDRPEINVEUTYSX3CZZQKBY5U3FE`
     <https://stellar.expert/explorer/public/contract/CBDS2YSLATINQVUDG5Y5HV4KQBEAVFDRPEINVEUTYSX3CZZQKBY5U3FE>

2. **A proof-of-KYC verified on mainnet, real tx:**
   `83ee1697486a24c3fd389b812f00c5693659cc3837f6fa653c42b62afc1751d6`
   <https://stellar.expert/explorer/public/tx/83ee1697486a24c3fd389b812f00c5693659cc3837f6fa653c42b62afc1751d6>
   The public signals carry **no PII**: only `ok`, the registered-set root, the
   nullifier, the year, and the age gate. Birth year and the sanction id stay
   private.

3. **Reproduce the verify yourself (no XLM, simulation only):**
   ```bash
   bash verify_mainnet.sh            # mandate proof -> mainnet verify=true
   ```

## What each circuit proves (nothing else revealed)

| circuit | proves | PII on-chain |
|---|---|---|
| `circuits/kyc.circom` | registered (Merkle) + age >= 18 + not sanctioned + nullifier | none |
| `circuits/mandate_sd.circom` | agent batch within caps + allowlist, total ElGamal-sealed to a regulator key | none |

Hash: **Poseidon** inside the circuit; the chain verifies **Groth16 over BN254**
(native since Protocol 25, "X-Ray"). The "feature on top of the X-Ray primitive"
the SDF docs say is missing: selective-disclosure compliance bound to a real
payment, not a standalone PoC.

## How an anchor adopts this

The verifier contract is **generic**: it verifies any Groth16/BN254 proof. To gate
your own flow, install the verify call as a policy signer in your account's
`__check_auth` (a payment only authorizes if the KYC proof verifies). The issuer
of credentials in production is a licensed partner; the anchor never stores the CPF,
only the commitment.

## Honest status

Unaudited, single-contributor trusted setup, demo keys. The math + the mainnet
verify are real; harden the setup (a multi-party ceremony) and audit the circuit
before protecting real funds. MIT licensed: fork it, import it, build on it.
