const { buildPoseidon } = require("circomlibjs");
const fs = require("fs");

(async () => {
  const poseidon = await buildPoseidon();
  const F = poseidon.F;
  const H = (arr) => F.toObject(poseidon(arr));

  const DEPTH = 10;
  // secrets
  const nullifier = 11111n, secret = 22222n, kycSecret = 33333n;
  const amount = 100n, threshold = 1000n;

  // leaf binds credential + kyc
  const leaf = H([nullifier, secret, kycSecret]);
  // nullifier hash (anti-reuse)
  const nullifierHash = H([nullifier]);

  // empty-tree zero subtree hashes
  const zeros = [0n];
  for (let i = 0; i < DEPTH; i++) zeros.push(H([zeros[i], zeros[i]]));

  // leaf at index 0: all siblings are zero-subtrees, all indices 0
  const pathElements = [], pathIndices = [];
  let cur = leaf;
  for (let i = 0; i < DEPTH; i++) {
    pathElements.push(zeros[i].toString());
    pathIndices.push("0");
    cur = H([cur, zeros[i]]);
  }
  const root = cur;

  const input = {
    amount: amount.toString(),
    secret: secret.toString(),
    nullifier: nullifier.toString(),
    kycSecret: kycSecret.toString(),
    pathElements,
    pathIndices,
    root: root.toString(),
    nullifierHash: nullifierHash.toString(),
    threshold: threshold.toString(),
  };
  fs.writeFileSync("build/input_v2.json", JSON.stringify(input, null, 2));
  console.log("input_v2.json escrito. root:", root.toString().slice(0,20)+"...", "| nullifierHash ok");
})();
