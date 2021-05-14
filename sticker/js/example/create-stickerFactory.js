const { Connection, Account, clusterApiUrl } = require("@solana/web3.js");

const { createStickerFactory } = require("./../client/sticker.js");

const NETWORK = clusterApiUrl("devnet");
const fs = require("fs");
const keyPath = "test_wallet.json";
const pk = JSON.parse(fs.readFileSync(keyPath));
const PAYER_ACCOUNT = new Account(pk);

(async function () {
  const connection = new Connection(NETWORK);

  let stickerFactoryOwner = new Account();

  let stickerFactoryAccount = await createStickerFactory(
    connection,
    PAYER_ACCOUNT,
    stickerFactoryOwner
  );

  console.log(
    "New StickerFactory account was created and initialized: ",
    stickerFactoryAccount.publicKey.toBase58()
  );
})();
