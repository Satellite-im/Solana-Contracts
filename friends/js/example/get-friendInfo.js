const {
  Connection,
  Account,
  clusterApiUrl,
  PublicKey,
} = require("@solana/web3.js");

const { createFriendInfo, getFriendInfo } = require("./../client/friends.js");

const { waitForAccount } = require("./../client/helper.js");

const NETWORK = clusterApiUrl("devnet");
const fs = require("fs");
const keyPath = "test_wallet.json";
const pk = JSON.parse(fs.readFileSync(keyPath));
const PAYER_ACCOUNT = new Account(pk);

(async function () {
  const connection = new Connection(NETWORK);

  let userAccount = new Account();

  console.log("User account was created: ", userAccount.publicKey.toBase58());

  let friendInfoAccount = await createFriendInfo(
    connection,
    PAYER_ACCOUNT,
    userAccount
  );

  console.log(
    "New FriendInfo account was created and initialized: ",
    friendInfoAccount.toBase58()
  );

  await waitForAccount(connection, friendInfoAccount);

  let friendInfoData = await getFriendInfo(connection, friendInfoAccount);

  console.log(`FriendInfo account data:`);
  console.log({
    requests_incoming: friendInfoData.requests_incoming,
    requests_outgoing: friendInfoData.requests_outgoing,
    friends: friendInfoData.friends,
    user: new PublicKey(Buffer.from(friendInfoData.user)).toBase58(),
  });
})();
