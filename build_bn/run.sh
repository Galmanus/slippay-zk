set -e
cd /home/galmanus/slippay-zk/build_bn
echo "=== ptau bn128 power 13 ==="
snarkjs powersoftau new bn128 13 pot_0000.ptau 2>&1 | tail -1
snarkjs powersoftau contribute pot_0000.ptau pot_0001.ptau --name="s" -e="e1" 2>&1 | tail -1
snarkjs powersoftau prepare phase2 pot_0001.ptau pot_final.ptau 2>&1 | tail -1
echo "=== setup + prove ==="
snarkjs groth16 setup mandate.r1cs pot_final.ptau m_0000.zkey 2>&1 | tail -1
snarkjs zkey contribute m_0000.zkey m_final.zkey --name="s2" -e="e2" 2>&1 | tail -1
snarkjs zkey export verificationkey m_final.zkey vk.json 2>&1 | tail -1
node mandate_js/generate_witness.js mandate_js/mandate.wasm input_mandate.json w.wtns
snarkjs groth16 prove m_final.zkey w.wtns proof.json public.json 2>&1 | tail -1
echo "--- VERIFY OFFCHAIN ---"; snarkjs groth16 verify vk.json public.json proof.json 2>&1 | tail -1
