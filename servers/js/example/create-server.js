const { Connection, Account, clusterApiUrl } = require("@solana/web3.js");
const { createDweller, createServer } = require("./../client/server.js");
const { waitForAccount } = require("./../client/helper.js");

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
    "test_name"
  );

  await waitForAccount(connection, dwellerAccount.publicKey);

  let server = await createServer(
    connection,
    PAYER_ACCOUNT,
    dwellerAccount,
    "test_name"
  );
  console.log("New server account: ", server.publicKey.toBase58());
})();
