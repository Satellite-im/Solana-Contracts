const {
  Connection,
  Account,
  clusterApiUrl,
  PublicKey,
} = require("@solana/web3.js");

const {
  createStickerFactory,
  getStickerFactory,
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

  let stickerFactoryData = await getStickerFactory(
    connection,
    stickerFactoryAccount.publicKey
  );

  console.log(
    "New StickerFactory account was created and initialized: ",
    stickerFactoryAccount.publicKey.toBase58()
  );
  console.log("Sticker factory account data:");
  console.log({
    artist_count: stickerFactoryData.artist_count,
    sticker_count: stickerFactoryData.sticker_count,
    owner: new PublicKey(Buffer.from(stickerFactoryData.owner)).toBase58(),
  });
})();
