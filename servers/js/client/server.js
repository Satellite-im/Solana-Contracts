const {
  SystemProgram,
  Transaction,
  TransactionInstruction,
  PublicKey,
  Account,
  sendAndConfirmTransaction,
  SYSVAR_RENT_PUBKEY,
} = require('@solana/web3.js');
const {
  encodeInstructionData,
  dwellerAccountLayout,
} = require("./layout");
const {stringToBuffer} = require("./helper");

const SERVER_PROGRAM_ID = new PublicKey(
  '5tg9sWeqFxv4mQuSKnoQVHW2iJLfa8XjEGFJyGeJ4nsT'
);

function initializeDweller(dweller, name) {
  return new TransactionInstruction({
    keys: [
      {pubkey: dweller.publicKey, isSigner: true, isWritable: true},
    ],
    programId: SERVER_PROGRAM_ID,
    data:
      encodeInstructionData({
        initializeDweller: { name: stringToBuffer(name, 32) },
      })
  });
}

function initializeServer(dwellerOwner, server, dwellerServer, serverMember, name) {
  return new TransactionInstruction({
    keys: [
      {pubkey: dwellerOwner.publicKey, isSigner: true, isWritable: true},
      {pubkey: server.publicKey, isSigner: true, isWritable: false},
      {pubkey: dwellerServer.publicKey, isSigner: false, isWritable: true},
      {pubkey: serverMember.publicKey, isSigner: false, isWritable: true},
    ],
    programId: SERVER_PROGRAM_ID,
    data:
      encodeInstructionData({
        initializeServer: { name: stringToBuffer(name, 32) },
      })
  });
}

async function createDweller(connection, payerAccount, name) {
  const space = dwellerAccountLayout.span;
  const lamports = await connection.getMinimumBalanceForRentExemption(
    space,
  );

  const dweller = new Account();

  const transaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payerAccount.publicKey,
      newAccountPubkey: dweller.publicKey,
      lamports,
      space,
      programId: SERVER_PROGRAM_ID,
    })
  ).add(
    initializeDweller(dweller, name)
  );

  const result = await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount, dweller],
    {
      commitment: 'singleGossip',
      preflightCommitment: 'singleGossip',
    },
  );
  return dweller;
}

async function getDweller(connection, dwellerPubkey) {
  const accountInfo = await connection.getAccountInfo(dwellerPubkey);
  if (accountInfo === null) {
    throw 'Error: cannot find the account';
  }
  const info = dwellerAccountLayout.decode(Buffer.from(accountInfo.data));
  return info;
}

module.exports = {
  SERVER_PROGRAM_ID,
  createDweller,
  getDweller,
}