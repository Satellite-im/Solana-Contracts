const {
  Connection,
  Account,
  PublicKey,
  clusterApiUrl,
} = require("@solana/web3.js");
const { getDweller } = require("./../client/server.js");

const NETWORK = clusterApiUrl("devnet");

const DWELLER = new PublicKey("3J5cpbEkNYZZ2wfjb2WhtJW3ANsWTdLUHW7fXR96NgT7");

(async function () {
  const connection = new Connection(NETWORK);

  const info = await getDweller(connection, DWELLER);

  console.log(`Dweller info:`);
  console.log({
    version: info.version,
    servers: info.servers,
    name: Buffer.from(info.name).toString("utf-8").replace(/\0.*$/g, ""),
    photo_hash: Buffer.from(info.photo_hash).toString("hex"),
    status: Buffer.from(info.status).toString("utf-8").replace(/\0.*$/g, ""),
  });
})();
