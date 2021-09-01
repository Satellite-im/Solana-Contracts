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
  friendLayout,
} = require("./../client/layout.js");

const FRIENDS_PROGRAM_ID = new PublicKey(
  "BxX6o2HG5DWrJt2v8GMSWNG2V2NtxNbAUF3wdE5Ao5gS"
);

const FRIEND_SEED = "friend";

async function createDerivedAccount(
  connection,
  payerAccount,
  seedKey,
  seedString,
  params
) {
  let base;
  base = await PublicKey.findProgramAddress(
    [seedKey.toBytes(), params.createAccount.friend.friendKey],
    FRIENDS_PROGRAM_ID
  );
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

async function createFriend(connection, payerAccount, userFromAccount, userToAccount) {
  let params= { createAccount: { friend: { friendKey: userToAccount.publicKey.toBytes() } } };
  console.log(userFromAccount.publicKey.toString());
  console.log(userToAccount.publicKey.toString());
  console.log(userFromAccount.publicKey.toBytes());
  console.log(userToAccount.publicKey.toBytes());
  console.log(params);
  let friendKey = await createDerivedAccount(
    connection,
    payerAccount,
    userFromAccount.publicKey,
    FRIEND_SEED,
    params
  );

  return friendKey;
}

async function initFriendRequest(
  friendKey,
  friend2Key,
  userFromKey,
  userToKey,
  fromPaddedBuffer,
  toPaddedBuffer,
) {
  return new TransactionInstruction({
    keys: [
      { pubkey: friendKey, isSigner: false, isWritable: true },
      { pubkey: friend2Key, isSigner: false, isWritable: true },
      { pubkey: userFromKey, isSigner: true, isWritable: false },
      { pubkey: userToKey, isSigner: false, isWritable: true },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: FRIENDS_PROGRAM_ID,
    data: encodeInstructionData({
      makeRequest: { tex: [fromPaddedBuffer.slice(0, 32), fromPaddedBuffer.slice(32, 64), toPaddedBuffer.slice(0, 32), toPaddedBuffer.slice(32, 64)] },
    }),
  });
}

async function initAcceptFriendRequest(
  friendKey,
  userFromKey,
  userToKey,
  fromPaddedBuffer,
  toPaddedBuffer,
) {
  return new TransactionInstruction({
    keys: [
      { pubkey: friendKey, isSigner: false, isWritable: true },
      { pubkey: userFromKey, isSigner: false, isWritable: true },
      { pubkey: userToKey, isSigner: true, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: FRIENDS_PROGRAM_ID,
    data: encodeInstructionData({
      acceptRequest: { tex: [fromPaddedBuffer.slice(0, 32), fromPaddedBuffer.slice(32, 64), toPaddedBuffer.slice(0, 32), toPaddedBuffer.slice(32, 64)] },
    }),
  });
}

async function initDenyFriendRequest(
  friendKey,
  userFromKey,
  userToKey,
  fromPaddedBuffer,
  toPaddedBuffer,
) {
  return new TransactionInstruction({
    keys: [
      { pubkey: friendKey, isSigner: false, isWritable: true },
      { pubkey: userFromKey, isSigner: false, isWritable: true },
      { pubkey: userToKey, isSigner: true, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: FRIENDS_PROGRAM_ID,
    data: encodeInstructionData({
      denyRequest: { tex: [fromPaddedBuffer.slice(0, 32), fromPaddedBuffer.slice(32, 64), toPaddedBuffer.slice(0, 32), toPaddedBuffer.slice(32, 64)] },
    }),
  });
}

async function initRemoveFriendRequest(
  friendKey,
  userFromKey,
  userToKey,
  fromPaddedBuffer,
  toPaddedBuffer,
) {
  return new TransactionInstruction({
    keys: [
      { pubkey: friendKey, isSigner: false, isWritable: true },
      { pubkey: userFromKey, isSigner: true, isWritable: false },
      { pubkey: userToKey, isSigner: false, isWritable: true },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: FRIENDS_PROGRAM_ID,
    data: encodeInstructionData({
      removeRequest: { tex: [fromPaddedBuffer.slice(0, 32), fromPaddedBuffer.slice(32, 64), toPaddedBuffer.slice(0, 32), toPaddedBuffer.slice(32, 64)] },
    }),
  });
}

async function initRemoveFriend(
  friendKey,
  userFromKey,
  userToKey,
  fromPaddedBuffer,
  toPaddedBuffer,
) {
  return new TransactionInstruction({
    keys: [
      { pubkey: friendKey, isSigner: false, isWritable: true },
      { pubkey: userFromKey, isSigner: true, isWritable: false },
      { pubkey: userToKey, isSigner: false, isWritable: true },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: FRIENDS_PROGRAM_ID,
    data: encodeInstructionData({
      removeFriend: { tex: [fromPaddedBuffer.slice(0, 32), fromPaddedBuffer.slice(32, 64), toPaddedBuffer.slice(0, 32), toPaddedBuffer.slice(32, 64)] },
    }),
  });
}

async function createFriendRequest(
  connection,
  payerAccount,
  friendKey,
  friend2Key,
  userFromAccount,
  userToKey,
  fromPaddedBuffer,
  toPaddedBuffer,
) {
  let transaction = new Transaction().add(
    await initFriendRequest(
      friendKey,
      friend2Key,
      userFromAccount.publicKey,
      userToKey,
      fromPaddedBuffer,
      toPaddedBuffer,
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

async function acceptFriendRequest(
  connection,
  payerAccount,
  friendKey,
  userFromKey,
  userToAccount,
  fromPaddedBuffer,
  toPaddedBuffer,
) {
  let transaction = new Transaction().add(
    await initAcceptFriendRequest(
      friendKey,
      userFromKey,
      userToAccount.publicKey,
      fromPaddedBuffer,
      toPaddedBuffer,
    )
  );

  await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount, userToAccount],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
  return { result: true };
}

async function denyFriendRequest(
  connection,
  payerAccount,
  friendKey,
  userFromKey,
  userToAccount,
  fromPaddedBuffer,
  toPaddedBuffer,
) {
  let transaction = new Transaction().add(
    await initDenyFriendRequest(
      friendKey,
      userFromKey,
      userToAccount.publicKey,
      fromPaddedBuffer,
      toPaddedBuffer,
    )
  );

  await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount, userToAccount],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
  return { result: true };
}

async function removeFriendRequest(
  connection,
  payerAccount,
  friendKey,
  userFromAccount,
  userToKey,
  fromPaddedBuffer,
  toPaddedBuffer,
) {
  let transaction = new Transaction().add(
    await initRemoveFriendRequest(
      friendKey,
      userFromAccount.publicKey,
      userToKey,
      fromPaddedBuffer,
      toPaddedBuffer,
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

async function removeFriend(
  connection,
  payerAccount,
  friendKey,
  userFromAccount,
  userToKey,
  fromPaddedBuffer,
  toPaddedBuffer,
) {
  let transaction = new Transaction().add(
    await initRemoveFriend(
      friendKey,
      userFromAccount.publicKey,
      userToKey,
      fromPaddedBuffer,
      toPaddedBuffer,
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

async function getFriend(connection, friendKey) {
  const accountInfo = await connection.getAccountInfo(friendKey);
  if (accountInfo === null) {
    throw "Error: cannot find the account";
  }
  const info = friendLayout.decode(Buffer.from(accountInfo.data));
  return info;
}

module.exports = {
  createFriend,
  createFriendRequest,
  acceptFriendRequest,
  denyFriendRequest,
  removeFriendRequest,
  removeFriend,
  getFriend,
};
