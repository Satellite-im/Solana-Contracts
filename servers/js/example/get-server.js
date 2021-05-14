const {
  Connection,
  Account,
  clusterApiUrl,
  PublicKey,
} = require("@solana/web3.js");
const {
  createDweller,
  createServer,
  getServer,
} = require("./../client/server.js");
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

  console.log("Dweller account created: ", dwellerAccount.publicKey);

  await waitForAccount(connection, dwellerAccount.publicKey);

  let server = await createServer(
    connection,
    PAYER_ACCOUNT,
    dwellerAccount,
    "test_name"
  );

  await waitForAccount(connection, server.publicKey);

  let serverInfo = await getServer(connection, server.publicKey);
  console.log(`Server info:`);
  console.log({
    version: serverInfo.version,
    owner: new PublicKey(Buffer.from(serverInfo.owner)).toBase58(),
    name: Buffer.from(serverInfo.name).toString("utf-8").replace(/\0.*$/g, ""),
    photo_hash: Buffer.from(serverInfo.photo_hash).toString("hex"),
    db_hash: Buffer.from(serverInfo.db_hash)
      .toString("utf-8")
      .replace(/\0.*$/g, ""),
    members: serverInfo.members,
    member_statuses: serverInfo.member_statuses,
    administrators: serverInfo.administrators,
    channels: serverInfo.channels,
    groups: serverInfo.groups,
  });
})();
