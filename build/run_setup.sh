set -e
cd /home/galmanus/slippay-zk/build
echo "=== powers of tau (bls12-381, 2^12) ==="
snarkjs powersoftau new bls12-381 12 pot12_0000.ptau -v 2>&1 | tail -1
snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="slippay1" -e="slippayzk entropy 1" 2>&1 | tail -1
snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau -v 2>&1 | tail -1
echo "=== groth16 setup + zkey contribute ==="
snarkjs groth16 setup compliance.r1cs pot12_final.ptau compliance_0000.zkey 2>&1 | tail -1
snarkjs zkey contribute compliance_0000.zkey compliance_final.zkey --name="slippay-zkey-1" -e="slippayzk entropy 2" 2>&1 | tail -1
snarkjs zkey export verificationkey compliance_final.zkey verification_key.json 2>&1 | tail -1
echo "=== witness + prove + verify offchain ==="
node compliance_js/generate_witness.js compliance_js/compliance.wasm input.json witness.wtns
snarkjs groth16 prove compliance_final.zkey witness.wtns proof.json public.json 2>&1 | tail -1
echo "--- VERIFY OFFCHAIN ---"
snarkjs groth16 verify verification_key.json public.json proof.json 2>&1 | tail -2
echo "--- public.json ---"; cat public.json
