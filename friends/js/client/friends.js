const {
  SystemProgram,
  Transaction,
  TransactionInstruction,
  PublicKey,
  sendAndConfirmTransaction,
  SYSVAR_RENT_PUBKEY,
} = require("@solana/web3.js");

const {
  encodeInstructionData,
  friendInfoAccountLayout,
  requestAccountLayout,
} = require("./../client/layout.js");

const FRIENDS_PROGRAM_ID = new PublicKey(
  "BxX6o2HG5DWrJt2v8GMSWNG2V2NtxNbAUF3wdE5Ao5gS"
);

const FRIEND_INFO_SEED = "friendinfo";
const OUTGOING_REQUEST = "outgoing";
const INCOMING_REQUEST = "incoming";
const FRIEND_SEED = "friend";

async function createDerivedAccount(
  connection,
  payerAccount,
  seedKey,
  seedString,
  params
) {
  //Qui ho dovuto agggiungere l'if per gestire il caso del "createFriend"
  let base;
  if (params.createAccount.friend) {
     base = await PublicKey.findProgramAddress(
      [seedKey.toBytes(), seedKey.toBytes()],
      FRIENDS_PROGRAM_ID
    );
  } else {
   base = await PublicKey.findProgramAddress(
    [seedKey.toBytes()],
    FRIENDS_PROGRAM_ID
  );}
  let addressToCreate = await PublicKey.createWithSeed(
    base[0],
    seedString,
    FRIENDS_PROGRAM_ID
  );
  let instruction = new TransactionInstruction({
    keys: [
      { pubkey: payerAccount.publicKey, isSigner: true, isWritable: true },
      { pubkey: seedKey, isSigner: false, isWritable: false },
      { pubkey: base[0], isSigner: false, isWritable: false },
      { pubkey: addressToCreate, isSigner: false, isWritable: true },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: FRIENDS_PROGRAM_ID,
    data: encodeInstructionData(params),
  });
  let transaction = new Transaction().add(instruction);
  await sendAndConfirmTransaction(connection, transaction, [payerAccount], {
    commitment: "singleGossip",
    preflightCommitment: "singleGossip",
  });
  return addressToCreate;
}

async function initFriendInfo(friendInfoPubKey, userKey) {
  return new TransactionInstruction({
    keys: [
      { pubkey: friendInfoPubKey, isSigner: false, isWritable: true },
      { pubkey: userKey, isSigner: true, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: FRIENDS_PROGRAM_ID,
    data: encodeInstructionData({
      initFriendInfo: {},
    }),
  });
}

async function createFriendInfo(connection, payerAccount, userAccount) {
  let params = { createAccount: { friendInfo: {} } };
  let friendInfoKey = await createDerivedAccount(
    connection,
    payerAccount,
    userAccount.publicKey,
    FRIEND_INFO_SEED,
    params
  );

  let transaction = new Transaction().add(
    await initFriendInfo(friendInfoKey, userAccount.publicKey)
  );

  await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount, userAccount],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
  return friendInfoKey;
}

async function getFriendInfo(connection, friendInfoKey) {
  const accountInfo = await connection.getAccountInfo(friendInfoKey);
  if (accountInfo === null) {
    throw "Error: cannot find the account";
  }
  const info = friendInfoAccountLayout.decode(Buffer.from(accountInfo.data));
  return info;
}

// Nuova, da testare
async function createFriend(connection, payerAccount, userAccount) {
  let params= { createAccount: { friend: { key: userAccount.publicKey.toBytes() } } };
  console.log(userAccount.publicKey.toString());
  console.log(params);
  let friendKey = await createDerivedAccount(
    connection,
    payerAccount,
    userAccount.publicKey,
    FRIEND_SEED,
    params
  );

  return friendKey;
}

async function initFriendRequest(
  requestFromToKey,
  requestToFromKey,
  friendInfoFromKey,
  friendInfoToKey,
  userFromKey,
  paddedBuffer
) {
  return new TransactionInstruction({
    keys: [
      { pubkey: requestFromToKey, isSigner: false, isWritable: true },
      { pubkey: requestToFromKey, isSigner: false, isWritable: true },
      { pubkey: friendInfoFromKey, isSigner: false, isWritable: true },
      { pubkey: friendInfoToKey, isSigner: false, isWritable: true },
      { pubkey: userFromKey, isSigner: true, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: FRIENDS_PROGRAM_ID,
    data: encodeInstructionData({
      makeRequest: [paddedBuffer.slice(0, 32), paddedBuffer.slice(32, 64)],
    }),
  });
}

// Nuova, da testare
async function initAcceptFriendRequest(
  requestFromToKey,
  requestToFromKey,
  lastRequestFromToKey,
  lastRequestToFromKey,
  friendInfoFromKey,
  friendInfoToKey,
  friendToKey,
  friendFromKey,
  userFromKey,
  paddedBuffer
) {
  return new TransactionInstruction({
    keys: [
      { pubkey: requestFromToKey, isSigner: false, isWritable: true },
      { pubkey: requestToFromKey, isSigner: false, isWritable: true },
      { pubkey: lastRequestFromToKey, isSigner: false, isWritable: true },
      { pubkey: lastRequestToFromKey, isSigner: false, isWritable: true },
      { pubkey: friendInfoFromKey, isSigner: false, isWritable: true },
      { pubkey: friendInfoToKey, isSigner: false, isWritable: true },
      { pubkey: friendToKey, isSigner: false, isWritable: true },
      { pubkey: friendFromKey, isSigner: false, isWritable: true },
      { pubkey: userFromKey, isSigner: true, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: FRIENDS_PROGRAM_ID,
    data: encodeInstructionData({
      acceptRequest: [paddedBuffer.slice(0, 32), paddedBuffer.slice(32, 64)],
    }),
  });
}

async function createFriendRequest(
  connection,
  payerAccount,
  userFromAccount,
  userToKey,
  friendInfoFromKey,
  friendInfoToKey,
  paddedBuffer
) {
  let friendInfoFromData = await getFriendInfo(connection, friendInfoFromKey);
  let params = {
    createAccount: {
      requestOutgoing: { index: friendInfoFromData.requests_outgoing },
    },
  };
  let requestFromAccount = await createDerivedAccount(
    connection,
    payerAccount,
    userFromAccount.publicKey,
    friendInfoFromData.requests_outgoing + OUTGOING_REQUEST,
    params
  );

  let friendInfoToData = await getFriendInfo(connection, friendInfoToKey);
  params = {
    createAccount: {
      requestIncoming: { index: friendInfoToData.requests_incoming },
    },
  };
  let requestToAccount = await createDerivedAccount(
    connection,
    payerAccount,
    userToKey,
    friendInfoToData.requests_incoming + INCOMING_REQUEST,
    params
  );

  let transaction = new Transaction().add(
    await initFriendRequest(
      requestFromAccount,
      requestToAccount,
      friendInfoFromKey,
      friendInfoToKey,
      userFromAccount.publicKey,
      paddedBuffer
    )
  );

  await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount, userFromAccount],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
  return { outgoing: requestFromAccount, incoming: requestToAccount };
}

// Nuova, qui vanno creati i derived accounts, analogamente a createFriendRequest (non passate direttamente le chiavi), altrimenti non va il Programma, che fa il controllo incrociato sulle keys (riga 522 processor.rs)
async function acceptFriendRequest(
  connection,
  payerAccount,
  requestFromToKey,
  requestToFromKey,
  lastRequestFromToKey,
  lastRequestToFromKey,
  friendInfoFromKey,
  friendInfoToKey,
  friendToKey,
  friendFromKey,
  userFromAccount,
  paddedBuffer
) {
  
  let transaction = new Transaction().add(
    await initAcceptFriendRequest(
      requestFromToKey,
      requestToFromKey,
      lastRequestFromToKey,
      lastRequestToFromKey,
      friendInfoFromKey,
      friendInfoToKey,
      friendToKey,
      friendFromKey,
      userFromAccount.publicKey,
      paddedBuffer
    )
  );

  await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount, userFromAccount],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
  return { result: true };
}

async function getFriendRequest(connection, friendRequestKey) {
  const accountInfo = await connection.getAccountInfo(friendRequestKey);
  if (accountInfo === null) {
    throw "Error: cannot find the account";
  }
  const info = requestAccountLayout.decode(Buffer.from(accountInfo.data));
  return info;
}

module.exports = {
  createFriendInfo,
  createFriend,
  getFriendInfo,
  createFriendRequest,
  acceptFriendRequest,
  getFriendRequest,
};
