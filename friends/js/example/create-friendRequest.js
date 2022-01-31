const { Connection, Account, clusterApiUrl, PublicKey } = require("@solana/web3.js");

const {
  createFriend,
  createFriendRequest,
  acceptFriendRequest,
  denyFriendRequest,
  removeFriendRequest,
  removeFriend,
  getFriend,
} = require("./../client/friends.js");

const { waitForAccount, sleep } = require("./../client/helper.js");

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
  let userToAccount = new Account();

  let friend = await createFriend(connection, PAYER_ACCOUNT, userFromAccount, userToAccount);
  let friend2 = await createFriend(connection, PAYER_ACCOUNT, userToAccount, userFromAccount);

  console.log(friend.toString());
  console.log(friend2.toString());
  await waitForAccount(connection, friend);

  let friendData = await getFriend(
    connection,
    friend
  );

  console.log("Friend data before creation:");
  console.log({
    from: new PublicKey(Buffer.from(friendData.from)).toBase58(),
    status: friendData.status,
    to: new PublicKey(Buffer.from(friendData.to)).toBase58(),
    texf1: Buffer.from(friendData.textileFrom1),
    texf2: Buffer.from(friendData.textileFrom2),
    texf3: Buffer.from(friendData.textileFrom3),
    texf4: Buffer.from(friendData.textileFrom4),
    text1: Buffer.from(friendData.textileTo1),
    text2: Buffer.from(friendData.textileTo2),
    text3: Buffer.from(friendData.textileTo3),
    text4: Buffer.from(friendData.textileTo4),
  });

  let friendRequests = await createFriendRequest(
    connection,
    PAYER_ACCOUNT,
    friend,
    friend2,
    userFromAccount,
    userToAccount.publicKey,
    paddedBuffer,
    paddedBuffer
  );

  await sleep(20000);

  friendData = await getFriend(
    connection,
    friend
  );

  console.log("Friend data after creation:");
  console.log({
    from: new PublicKey(Buffer.from(friendData.from)).toBase58(),
    status: friendData.status,
    to: new PublicKey(Buffer.from(friendData.to)).toBase58(),
    texf1: Buffer.from(friendData.textileFrom1),
    texf2: Buffer.from(friendData.textileFrom2),
    texf3: Buffer.from(friendData.textileFrom3),
    texf4: Buffer.from(friendData.textileFrom4),
    text1: Buffer.from(friendData.textileTo1),
    text2: Buffer.from(friendData.textileTo2),
    text3: Buffer.from(friendData.textileTo3),
    text4: Buffer.from(friendData.textileTo4),
  });

  let acceptRequest = await acceptFriendRequest(
    connection,
    PAYER_ACCOUNT,
    friend,
    userFromAccount.publicKey,
    userToAccount,
    paddedBuffer,
    paddedBuffer
  );

  await sleep(20000);

  friendData = await getFriend(
    connection,
    friend
  );

  console.log("Friend data after accept:");
  console.log({
    from: new PublicKey(Buffer.from(friendData.from)).toBase58(),
    status: friendData.status,
    to: new PublicKey(Buffer.from(friendData.to)).toBase58(),
    texf1: Buffer.from(friendData.textileFrom1),
    texf2: Buffer.from(friendData.textileFrom2),
    texf3: Buffer.from(friendData.textileFrom3),
    texf4: Buffer.from(friendData.textileFrom4),
    text1: Buffer.from(friendData.textileTo1),
    text2: Buffer.from(friendData.textileTo2),
    text3: Buffer.from(friendData.textileTo3),
    text4: Buffer.from(friendData.textileTo4),
  });

  let removeRequest = await removeFriend(
    connection,
    PAYER_ACCOUNT,
    friend,
    userFromAccount,
    userToAccount.publicKey
  );

  await sleep(20000);

  friendData = await getFriend(
    connection,
    friend
  );

  console.log("Friend data after remove:");
  console.log({
    from: new PublicKey(Buffer.from(friendData.from)).toBase58(),
    status: friendData.status,
    to: new PublicKey(Buffer.from(friendData.to)).toBase58(),
    texf1: Buffer.from(friendData.textileFrom1),
    texf2: Buffer.from(friendData.textileFrom2),
    texf3: Buffer.from(friendData.textileFrom3),
    texf4: Buffer.from(friendData.textileFrom4),
    text1: Buffer.from(friendData.textileTo1),
    text2: Buffer.from(friendData.textileTo2),
    text3: Buffer.from(friendData.textileTo3),
    text4: Buffer.from(friendData.textileTo4),
  });

})();
