const { Connection, Account, clusterApiUrl } = require("@solana/web3.js");

const {
  createFriendInfo,
  createFriendRequest,
} = require("./../client/friends.js");

const { waitForAccount } = require("./../client/helper.js");

const NETWORK = clusterApiUrl("devnet");
const fs = require("fs");
const keyPath = "test_wallet.json";
const pk = JSON.parse(fs.readFileSync(keyPath));
const PAYER_ACCOUNT = new Account(pk);

(async function () {
  const connection = new Connection(NETWORK);

  let userFromAccount = new Account();

  let friendInfoFromAccount = await createFriendInfo(
    connection,
    PAYER_ACCOUNT,
    userFromAccount
  );

  console.log(
    "New FriendInfo account was created and initialized: ",
    friendInfoFromAccount.toBase58()
  );

  let userToAccount = new Account();

  let friendInfoToAccount = await createFriendInfo(
    connection,
    PAYER_ACCOUNT,
    userToAccount
  );

  console.log(
    "New FriendInfo account was created and initialized: ",
    friendInfoToAccount.toBase58()
  );

  await waitForAccount(connection, friendInfoToAccount);

  let friendRequests = await createFriendRequest(
    connection,
    PAYER_ACCOUNT,
    userFromAccount,
    userToAccount.publicKey,
    friendInfoFromAccount,
    friendInfoToAccount
  );

  console.log(
    `New friend request was created.\nIncoming: ${friendRequests.incoming}\nOutgoing: ${friendRequests.outgoing}`
  );
})();
