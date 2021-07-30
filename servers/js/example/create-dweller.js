const { Connection, Account, clusterApiUrl } = require("@solana/web3.js");
const { createDweller } = require("./../client/server.js");

const NETWORK = clusterApiUrl("devnet");
const fs = require("fs");
const keyPath = "test_wallet.json";
const pk = JSON.parse(fs.readFileSync(keyPath));
const PAYER_ACCOUNT = new Account(pk);

(async function () {
  const connection = new Connection(NETWORK);

  const dwellerAccount = await createDweller(
    connection,
    PAYER_ACCOUNT,
    "test_name",
    "BE62DF6F8308796B370685A5EDFE8EA25BC48524FF356639A5FB5E5504B3B2D9",
    "A long and passionate description which reflects user's personality"
  );

  console.log(`Dweller created with pubkey ${dwellerAccount.publicKey}`);
})();
