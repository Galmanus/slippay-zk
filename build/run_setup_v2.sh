set -e
cd /home/galmanus/slippay-zk/build
echo "=== ptau bls12-381 power 13 ==="
snarkjs powersoftau new bls12-381 13 pot13_0000.ptau 2>&1 | tail -1
snarkjs powersoftau contribute pot13_0000.ptau pot13_0001.ptau --name="slippay-v2" -e="slippay v2 entropy a" 2>&1 | tail -1
snarkjs powersoftau prepare phase2 pot13_0001.ptau pot13_final.ptau 2>&1 | tail -1
echo "=== groth16 setup v2 ==="
snarkjs groth16 setup compliance_v2.r1cs pot13_final.ptau v2_0000.zkey 2>&1 | tail -1
snarkjs zkey contribute v2_0000.zkey v2_final.zkey --name="slippay-v2-zkey" -e="slippay v2 entropy b" 2>&1 | tail -1
snarkjs zkey export verificationkey v2_final.zkey vk_v2.json 2>&1 | tail -1
echo "=== witness + prove + verify offchain ==="
node compliance_v2_js/generate_witness.js compliance_v2_js/compliance_v2.wasm input_v2.json witness_v2.wtns
snarkjs groth16 prove v2_final.zkey witness_v2.wtns proof_v2.json public_v2.json 2>&1 | tail -1
echo "--- VERIFY OFFCHAIN ---"
snarkjs groth16 verify vk_v2.json public_v2.json proof_v2.json 2>&1 | tail -2
echo "--- public_v2.json (root, nullifierHash, threshold, ok) ---"; cat public_v2.json
