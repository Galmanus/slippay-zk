set -e
cd /home/galmanus/slippay-zk/build_sd
echo "=== setup (reusa ptau bn128 power13) ==="
snarkjs groth16 setup mandate_sd.r1cs ../build_bn/pot_final.ptau sd_0000.zkey 2>&1 | tail -1
snarkjs zkey contribute sd_0000.zkey sd_final.zkey --name="sd" -e="sd entropy" 2>&1 | tail -1
snarkjs zkey export verificationkey sd_final.zkey vk_sd.json 2>&1 | tail -1
node mandate_sd_js/generate_witness.js mandate_sd_js/mandate_sd.wasm input_sd.json w_sd.wtns
snarkjs groth16 prove sd_final.zkey w_sd.wtns proof_sd.json public_sd.json 2>&1 | tail -1
echo "--- VERIFY OFFCHAIN ---"; snarkjs groth16 verify vk_sd.json public_sd.json proof_sd.json 2>&1 | tail -1
echo "--- public_sd.json (ok, ephemeralKey, encryptedTotal, mandato, regPubKey) ---"; cat public_sd.json
