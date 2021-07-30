const {
  SystemProgram,
  Transaction,
  TransactionInstruction,
  PublicKey,
  Account,
  sendAndConfirmTransaction,
  SYSVAR_RENT_PUBKEY,
} = require("@solana/web3.js");
const {
  encodeInstructionData,
  dwellerAccountLayout,
  serverAccountLayout,
} = require("./layout");
const { stringToBuffer } = require("./helper");

const SERVER_PROGRAM_ID = new PublicKey(
  "FGdpP9RSN3ZE8d1PXxiBXS8ThCsXdi342KmDwqSQ3ZBz"
);

const DWELLER_SERVER_SEED = "DwellerServer";

const SERVER_MEMBER_SEED = "ServerMember";

function initializeDweller(dweller, name, hash, status) {
  return new TransactionInstruction({
    keys: [{ pubkey: dweller.publicKey, isSigner: true, isWritable: true }],
    programId: SERVER_PROGRAM_ID,
    data: encodeInstructionData({
      initializeDweller: { name: stringToBuffer(name, 32),
                           hash: stringToBuffer(hash, 64),
                           status: stringToBuffer(status, 128) },
    }),
  });
}

function initializeServer(
  dwellerOwner,
  server,
  dwellerServer,
  serverMember,
  name
) {
  return new TransactionInstruction({
    keys: [
      { pubkey: dwellerOwner, isSigner: true, isWritable: true },
      { pubkey: server, isSigner: true, isWritable: false },
      { pubkey: dwellerServer, isSigner: false, isWritable: true },
      { pubkey: serverMember, isSigner: false, isWritable: true },
    ],
    programId: SERVER_PROGRAM_ID,
    data: encodeInstructionData({
      initializeServer: { name: stringToBuffer(name, 32) },
    }),
  });
}

async function createDweller(connection, payerAccount, name, hash, status) {
  const space = dwellerAccountLayout.span;
  const lamports = await connection.getMinimumBalanceForRentExemption(space);

  const dweller = new Account();

  const transaction = new Transaction()
    .add(
      SystemProgram.createAccount({
        fromPubkey: payerAccount.publicKey,
        newAccountPubkey: dweller.publicKey,
        lamports,
        space,
        programId: SERVER_PROGRAM_ID,
      })
    )
    .add(initializeDweller(dweller, name, hash, status));

  const result = await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount, dweller],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
  return dweller;
}

async function getDweller(connection, dwellerPubkey) {
  const accountInfo = await connection.getAccountInfo(dwellerPubkey);
  if (accountInfo === null) {
    throw "Error: cannot find the account";
  }
  const info = dwellerAccountLayout.decode(Buffer.from(accountInfo.data));
  return info;
}

async function createDerivedAccount(
  connection,
  payerAccount,
  seedKey,
  seedString,
  index,
  addressTypeValue
) {
  let base = await PublicKey.findProgramAddress(
    [seedKey.toBytes()],
    SERVER_PROGRAM_ID
  );
  let addressToCreate = await PublicKey.createWithSeed(
    base[0],
    seedString + index,
    SERVER_PROGRAM_ID
  );
  let params = { createDerivedAccount: {} };
  params.createDerivedAccount[addressTypeValue] = index;
  let instruction = new TransactionInstruction({
    keys: [
      { pubkey: payerAccount.publicKey, isSigner: true, isWritable: true },
      { pubkey: seedKey, isSigner: false, isWritable: false },
      { pubkey: base[0], isSigner: false, isWritable: false },
      { pubkey: addressToCreate, isSigner: false, isWritable: true },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: SERVER_PROGRAM_ID,
    data: encodeInstructionData(params),
  });

  let transaction = new Transaction().add(instruction);

  const result = await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
  return addressToCreate;
}

async function createServer(connection, payerAccount, dwellerAccount, name) {
  let server = new Account();
  let dwellerData = await getDweller(connection, dwellerAccount.publicKey);

  let dwellerServer = await createDerivedAccount(
    connection,
    payerAccount,
    dwellerAccount.publicKey,
    DWELLER_SERVER_SEED,
    dwellerData.servers,
    "dwellerServer"
  );
  console.log("DwellerServer account created: ", dwellerServer.toBase58());
  let serverMembers = 0;
  let serverMember = await createDerivedAccount(
    connection,
    payerAccount,
    server.publicKey,
    SERVER_MEMBER_SEED,
    serverMembers,
    "serverMember"
  );
  console.log("ServerMember account created: ", serverMember.toBase58());

  const space = serverAccountLayout.span;
  const lamports = await connection.getMinimumBalanceForRentExemption(space);

  const transaction = new Transaction()
    .add(
      SystemProgram.createAccount({
        fromPubkey: payerAccount.publicKey,
        newAccountPubkey: server.publicKey,
        lamports,
        space,
        programId: SERVER_PROGRAM_ID,
      })
    )
    .add(
      initializeServer(
        dwellerAccount.publicKey,
        server.publicKey,
        dwellerServer,
        serverMember,
        name
      )
    );

  const result = await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount, server, dwellerAccount],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
  return server;
}

async function getServer(connection, serverPubkey) {
  const accountInfo = await connection.getAccountInfo(serverPubkey);
  if (accountInfo === null) {
    throw "Error: cannot find the account";
  }
  const info = serverAccountLayout.decode(Buffer.from(accountInfo.data));
  return info;
}

module.exports = {
  SERVER_PROGRAM_ID,
  createDweller,
  getDweller,
  createServer,
  getServer,
};
