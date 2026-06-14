set -e
cd /home/galmanus/slippay-zk/build
snarkjs groth16 setup mandate.r1cs pot13_final.ptau mandate_0000.zkey 2>&1 | tail -1
snarkjs zkey contribute mandate_0000.zkey mandate_final.zkey --name="slippay-mandate" -e="mandate entropy" 2>&1 | tail -1
snarkjs zkey export verificationkey mandate_final.zkey vk_mandate.json 2>&1 | tail -1
node mandate_js/generate_witness.js mandate_js/mandate.wasm input_mandate.json witness_mandate.wtns
snarkjs groth16 prove mandate_final.zkey witness_mandate.wtns proof_mandate.json public_mandate.json 2>&1 | tail -1
echo "--- VERIFY OFFCHAIN ---"
snarkjs groth16 verify vk_mandate.json public_mandate.json proof_mandate.json 2>&1 | tail -2
echo "--- public_mandate.json ---"; cat public_mandate.json
