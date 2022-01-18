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
  "3oLLwXWbtfNRrsMvZCF63P5hSvmQU1biaKsvdqfcxEpM"
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
    [seedKey.toBytes(), params.createAccount],
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
  let params= { createAccount: userToAccount.publicKey.toBytes() };
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
  fromPaddedBuffer1,
  fromPaddedBuffer2,
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
      makeRequest: { tex: [fromPaddedBuffer1.slice(0, 32), fromPaddedBuffer1.slice(32, 64), fromPaddedBuffer2.slice(0, 32), fromPaddedBuffer2.slice(32, 64)] },
    }),
  });
}

async function initAcceptFriendRequest(
  friendKey,
  userFromKey,
  userToKey,
  toPaddedBuffer1,
  toPaddedBuffer2,
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
      acceptRequest: { tex: [toPaddedBuffer1.slice(0, 32), toPaddedBuffer1.slice(32, 64), toPaddedBuffer2.slice(0, 32), toPaddedBuffer2.slice(32, 64)] },
    }),
  });
}

async function initDenyFriendRequest(
  friendKey,
  userFromKey,
  userToKey,
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
      denyRequest: {},
    }),
  });
}

async function initRemoveFriendRequest(
  friendKey,
  userFromKey,
  userToKey,
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
      removeRequest: {},
    }),
  });
}

async function initRemoveFriend(
  friendKey,
  userFromKey,
  userToKey,
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
      removeFriend: {},
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
  fromPaddedBuffer1,
  fromPaddedBuffer2
) {
  let transaction = new Transaction().add(
    await initFriendRequest(
      friendKey,
      friend2Key,
      userFromAccount.publicKey,
      userToKey,
      fromPaddedBuffer1,
      fromPaddedBuffer2,
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
  toPaddedBuffer1,
  toPaddedBuffer2,
) {
  let transaction = new Transaction().add(
    await initAcceptFriendRequest(
      friendKey,
      userFromKey,
      userToAccount.publicKey,
      toPaddedBuffer1,
      toPaddedBuffer2,
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
) {
  let transaction = new Transaction().add(
    await initDenyFriendRequest(
      friendKey,
      userFromKey,
      userToAccount.publicKey,
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
) {
  let transaction = new Transaction().add(
    await initRemoveFriendRequest(
      friendKey,
      userFromAccount.publicKey,
      userToKey,
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
) {
  let transaction = new Transaction().add(
    await initRemoveFriend(
      friendKey,
      userFromAccount.publicKey,
      userToKey,
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
