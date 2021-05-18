const { Connection, Account, clusterApiUrl } = require("@solana/web3.js");

const { createFriendInfo } = require("./../client/friends.js");

const NETWORK = clusterApiUrl("devnet");
const fs = require("fs");
const keyPath = "test_wallet.json";
const pk = JSON.parse(fs.readFileSync(keyPath));
const PAYER_ACCOUNT = new Account(pk);

(async function () {
  const connection = new Connection(NETWORK);

  let userAccount = new Account();

  let friendInfoAccount = await createFriendInfo(
    connection,
    PAYER_ACCOUNT,
    userAccount
  );

  console.log(
    "New FriendInfo account was created and initialized: ",
    friendInfoAccount.toBase58()
  );
})();
