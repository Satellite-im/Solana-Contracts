const { Connection, Account, clusterApiUrl } = require("@solana/web3.js");

const {
  createStickerFactory,
  createArtist,
} = require("./../client/sticker.js");

const { waitForAccount } = require("./../client/helper.js");

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

  await waitForAccount(connection, stickerFactoryAccount.publicKey);

  console.log(
    "New StickerFactory account was created and initialized: ",
    stickerFactoryAccount.publicKey.toBase58()
  );

  let artistUserAccount = new Account();
  let userTokenAccount = new Account();
  let userTokenAccountOwner = new Account();
  let data = {
    name: new Array(32).fill(2, 0, 32),
    signature: new Array(256).fill(3, 0, 256),
    description: new Array(256).fill(4, 0, 256),
  };

  let artistKey = await createArtist(
    connection,
    PAYER_ACCOUNT,
    artistUserAccount,
    userTokenAccount,
    userTokenAccountOwner,
    stickerFactoryAccount,
    stickerFactoryOwner,
    data
  );

  console.log("New artist was created and initialized: ", artistKey.toBase58());
})();
