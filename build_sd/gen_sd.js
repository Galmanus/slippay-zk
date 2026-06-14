const { buildBabyjub } = require("circomlibjs");
const fs = require("fs");
(async () => {
  const bj = await buildBabyjub();
  const F = bj.F;
  const d = 987654321n;     // regulator private key (kept by regulator)
  const nonce = 555555n;    // ephemeral nonce
  const pk = bj.mulPointEscalar(bj.Base8, d);
  const input = {
    amounts: ["100","200","150","250","100","300","200","150"],
    recipients: ["11","22","33","44","11","22","33","44"],
    nonceKey: nonce.toString(),
    perPaymentCap: "500",
    monthlyCap: "2000",
    allowed: ["11","22","33","44"],
    regPubKey: [ F.toObject(pk[0]).toString(), F.toObject(pk[1]).toString() ],
  };
  fs.writeFileSync("build_sd/input_sd.json", JSON.stringify(input, null, 2));
  console.log("input_sd.json escrito | regPubKey ok | total real=1450 (cifrado)");
})();
