// Regulator-side decryption: recover the real monthly total from the public ciphertext
// using the regulator's Baby Jubjub private key. Proves the full selective-disclosure loop.
const { buildBabyjub } = require("circomlibjs");
const fs = require("fs");
(async () => {
  const bj = await buildBabyjub();
  const F = bj.F;
  const pub = JSON.parse(fs.readFileSync("build_sd/public_sd.json"));
  // public_sd.json = [ok, ephX, ephY, encX, encY, ...mandate...]
  const eph = [F.e(pub[1]), F.e(pub[2])];          // ephemeralKey = nonce.G
  const enc = [F.e(pub[3]), F.e(pub[4])];          // encryptedTotal = total.G + nonce.PK
  const d = 987654321n;                             // regulator private key

  // shared = d * ephemeralKey ; M = encryptedTotal - shared
  const shared = bj.mulPointEscalar(eph, d);
  const negShared = [F.neg(shared[0]), shared[1]];  // -(x,y) = (-x, y) on twisted Edwards
  const M = bj.addPoint(enc, negShared);            // = total.G

  // recover total via discrete log over the bounded range (amounts are capped)
  let recovered = null;
  let acc = [F.e(0n), F.e(1n)]; // identity
  for (let t = 0; t <= 5000; t++) {
    if (F.eq(acc[0], M[0]) && F.eq(acc[1], M[1])) { recovered = t; break; }
    acc = bj.addPoint(acc, bj.Base8);
  }
  console.log("REGULATOR_RECOVERED_TOTAL =", recovered, "(real total was 1450, sum of the 8 private payments)");
  console.log("match:", recovered === 1450);
})();
