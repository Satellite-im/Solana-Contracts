const { Connection, Account, clusterApiUrl } = require("@solana/web3.js");

const {
  createFriendInfo,
  createFriendRequest,
  createFriend,
  acceptFriendRequest
} = require("./../client/friends.js");

const { waitForAccount } = require("./../client/helper.js");

const NETWORK = 'http://127.0.0.1:8899';//clusterApiUrl("devnet");
const fs = require("fs");
const keyPath = "test_wallet.json";
const pk = JSON.parse(fs.readFileSync(keyPath));
const PAYER_ACCOUNT = new Account(pk);

const textileMailboxId =
  "bafkwqw5h6zlko43enhmrrlksx3fhitmojzpnwtagbrjcflm737btxbq";

const paddedBuffer = Buffer.from(textileMailboxId.padStart(64, "0"));

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
    friendInfoToAccount,
    paddedBuffer
  );

  console.log(
    `New friend request was created.\nIncoming: ${friendRequests.incoming}\nOutgoing: ${friendRequests.outgoing}`
  );

  let friendFrom = await createFriend(connection, PAYER_ACCOUNT, userFromAccount);
  let friendTo = await createFriend(connection, PAYER_ACCOUNT, userToAccount);

  console.log(friendFrom, friendTo);
  let acceptFriend = await acceptFriendRequest(
    connection,
    PAYER_ACCOUNT,
    friendRequests.incoming,
    friendRequests.outgoing,
    friendRequests.incoming,
    friendRequests.outgoing,
    friendInfoFromAccount,
    friendInfoToAccount,
    friendTo,
    friendFrom,
    userFromAccount,
    paddedBuffer
  );
})();
