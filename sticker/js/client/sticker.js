const {
  SystemProgram,
  Transaction,
  TransactionInstruction,
  PublicKey,
  sendAndConfirmTransaction,
  SYSVAR_RENT_PUBKEY,
  Account,
} = require("@solana/web3.js");

const BufferLayout = require("buffer-layout");

const {
  encodeInstructionData,
  stickerFactoryAccountLayout,
  artistAccountLayout,
  tokenAccountLayout,
} = require("./../client/layout.js");

const { waitForAccount } = require("./../client/helper.js");

const STICKER_PROGRAM_ID = new PublicKey(
  "6fKUpiC3zAZZ3Y2KsidwECZAivpW4K3NVNJK8UEAC4DR"
);

const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);

const NATIVE_MINT_ID = new PublicKey(
  "So11111111111111111111111111111111111111112"
);

const ARTIST_SEED = "artist";
const STICKER_SEED = "sticker";

async function createDerivedAccount(
  connection,
  payerAccount,
  seedKey,
  seedString,
  params
) {
  let base = await PublicKey.findProgramAddress(
    [seedKey.toBytes()],
    STICKER_PROGRAM_ID
  );
  let addressToCreate = await PublicKey.createWithSeed(
    base[0],
    seedString,
    STICKER_PROGRAM_ID
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
    programId: STICKER_PROGRAM_ID,
    data: encodeInstructionData(params),
  });

  let transaction = new Transaction().add(instruction);

  await sendAndConfirmTransaction(connection, transaction, [payerAccount], {
    commitment: "singleGossip",
    preflightCommitment: "singleGossip",
  });
  return addressToCreate;
}

function createInitTokenAccountInstruction(programId, mint, account, owner) {
  const keys = [
    { pubkey: account, isSigner: false, isWritable: true },
    { pubkey: mint, isSigner: false, isWritable: false },
    { pubkey: owner, isSigner: false, isWritable: false },
    { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
  ];
  const dataLayout = BufferLayout.struct([BufferLayout.u8("instruction")]);
  const data = Buffer.alloc(dataLayout.span);
  dataLayout.encode(
    {
      instruction: 1, // InitializeAccount instruction
    },
    data
  );

  return new TransactionInstruction({
    keys,
    programId,
    data,
  });
}

function initializeStickerFactory(stickerFactoryKey, ownerKey) {
  return new TransactionInstruction({
    keys: [
      { pubkey: stickerFactoryKey, isSigner: false, isWritable: true },
      { pubkey: ownerKey, isSigner: true, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: STICKER_PROGRAM_ID,
    data: encodeInstructionData({ createStickerFactory: {} }),
  });
}

async function createStickerFactory(
  connection,
  payerAccount,
  stickerFactoryOwnerAccount
) {
  let stickerFactoryAccount = new Account();

  const space = stickerFactoryAccountLayout.span;
  const lamports = await connection.getMinimumBalanceForRentExemption(space);

  const transaction = new Transaction()
    .add(
      SystemProgram.createAccount({
        fromPubkey: payerAccount.publicKey,
        newAccountPubkey: stickerFactoryAccount.publicKey,
        lamports,
        space,
        programId: STICKER_PROGRAM_ID,
      })
    )
    .add(
      initializeStickerFactory(
        stickerFactoryAccount.publicKey,
        stickerFactoryOwnerAccount.publicKey
      )
    );

  await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount, stickerFactoryAccount, stickerFactoryOwnerAccount],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
  return stickerFactoryAccount;
}

async function getStickerFactory(connection, stickerFactoryKey) {
  const accountInfo = await connection.getAccountInfo(stickerFactoryKey);
  if (accountInfo === null) {
    throw "Error: cannot find the account";
  }
  const info = stickerFactoryAccountLayout.decode(
    Buffer.from(accountInfo.data)
  );
  return info;
}

function registerArtist(
  userKey,
  userTokenKey,
  artistKey,
  stickerFactoryOwnerKey,
  stickerFactoryKey,
  data
) {
  return new TransactionInstruction({
    keys: [
      { pubkey: userKey, isSigner: false, isWritable: false },
      { pubkey: userTokenKey, isSigner: false, isWritable: false },
      { pubkey: artistKey, isSigner: false, isWritable: true },
      { pubkey: stickerFactoryOwnerKey, isSigner: true, isWritable: false },
      { pubkey: stickerFactoryKey, isSigner: false, isWritable: true },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId: STICKER_PROGRAM_ID,
    data: encodeInstructionData({ registerArtist: data }),
  });
}

async function createTokenAccount(
  connection,
  payerAccount,
  accountToCreate,
  accountOwner,
  lamports,
  space
) {
  const transaction = new Transaction();
  transaction.add(
    SystemProgram.createAccount({
      fromPubkey: payerAccount.publicKey,
      newAccountPubkey: accountToCreate.publicKey,
      lamports: lamports,
      space: space,
      programId: TOKEN_PROGRAM_ID,
    })
  );
  transaction.add(
    createInitTokenAccountInstruction(
      TOKEN_PROGRAM_ID,
      NATIVE_MINT_ID,
      accountToCreate.publicKey,
      accountOwner.publicKey
    )
  );

  await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount, accountToCreate],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
}

async function createArtist(
  connection,
  payerAccount,
  artistUserAccount,
  userTokenAccount,
  userTokenAccountOwner,
  stickerFactoryAccount,
  stickerFactoryOwnerAccount,
  data
) {
  let stickerFactoryData = await getStickerFactory(
    connection,
    stickerFactoryAccount.publicKey
  );

  let artistKey = await createDerivedAccount(
    connection,
    payerAccount,
    stickerFactoryAccount.publicKey,
    stickerFactoryData.artist_count + ARTIST_SEED,
    { createAccount: { artist: {} } }
  );

  let lamports = await connection.getMinimumBalanceForRentExemption(
    tokenAccountLayout.span
  );

  await createTokenAccount(
    connection,
    payerAccount,
    userTokenAccount,
    userTokenAccountOwner,
    lamports,
    tokenAccountLayout.span
  );

  await waitForAccount(connection, userTokenAccount.publicKey); // wait till we can use user token account

  let transaction = new Transaction();
  transaction.add(
    registerArtist(
      artistUserAccount.publicKey,
      userTokenAccount.publicKey,
      artistKey,
      stickerFactoryOwnerAccount.publicKey,
      stickerFactoryAccount.publicKey,
      data
    )
  );

  await sendAndConfirmTransaction(
    connection,
    transaction,
    [payerAccount, stickerFactoryOwnerAccount],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );

  return artistKey;
}

async function getArtistAccountData(connection, artistKey) {
  const accountInfo = await connection.getAccountInfo(artistKey);
  if (accountInfo === null) {
    throw "Error: cannot find the account";
  }
  const info = artistAccountLayout.decode(
    Buffer.from(accountInfo.data)
  );
  return info;
}

module.exports = {
  createStickerFactory,
  getStickerFactory,
  createArtist,
  getArtistAccountData,
};
