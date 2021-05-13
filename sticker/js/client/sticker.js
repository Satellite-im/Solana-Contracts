const {
    SystemProgram,
    Transaction,
    TransactionInstruction,
    PublicKey,
    sendAndConfirmTransaction,
    SYSVAR_RENT_PUBKEY,
    Account,
} = require('@solana/web3.js');

const {
    encodeInstructionData,
    stickerFactoryAccountLayout,
    artistAccountLayout,
} = require('./../client/layout.js');

const STICKER_PROGRAM_ID = new PublicKey(
    '6fKUpiC3zAZZ3Y2KsidwECZAivpW4K3NVNJK8UEAC4DR'
);

const ARTIST_SEED = 'artist';
const STICKER_SEED = 'sticker';

async function createDerivedAccount(connection, payerAccount, seedKey, seedString, params) {
    let base = await PublicKey.findProgramAddress([seedKey.toBytes()], STICKER_PROGRAM_ID);
    let addressToCreate = await PublicKey.createWithSeed(base[0], seedString, STICKER_PROGRAM_ID);
    let instruction = new TransactionInstruction({
      keys: [
        {pubkey: payerAccount.publicKey, isSigner: true, isWritable: true},
        {pubkey: seedKey, isSigner: false, isWritable: false},
        {pubkey: base[0], isSigner: false, isWritable: false},
        {pubkey: addressToCreate, isSigner: false, isWritable: true},
        {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
      ],
      programId: STICKER_PROGRAM_ID,
      data: encodeInstructionData(params)
    });
  
    let transaction = new Transaction().add(instruction);
  
    await sendAndConfirmTransaction(
      connection,
      transaction,
      [payerAccount],
      {
        commitment: 'singleGossip',
        preflightCommitment: 'singleGossip',
      },
    );
    return addressToCreate;
}

async function initializeStickerFactory(stickerFactoryKey, ownerKey) {
    return new TransactionInstruction({
        keys: [
            {pubkey: stickerFactoryKey, isSigner: false, isWritable: true},
            {pubkey: ownerKey, isSigner: true, isWritable: false},
            {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
        ],
        programId: STICKER_PROGRAM_ID,
        data: encodeInstructionData({createStickerFactory: {}})
    });
}

async function createStickerFactory(connection, payerAccount, stickerFactoryOwnerAccount) {
    let stickerFactoryAccount = new Account();
    
    const space = stickerFactoryAccountLayout.span;
    const lamports = await connection.getMinimumBalanceForRentExemption(
        space,
    );

    const transaction = new Transaction().add(
        SystemProgram.createAccount({
          fromPubkey: payerAccount.publicKey,
          newAccountPubkey: stickerFactoryAccount.publicKey,
          lamports,
          space,
          programId: STICKER_PROGRAM_ID,
        })
      ).add(
        await initializeStickerFactory(stickerFactoryAccount.publicKey, stickerFactoryOwnerAccount.publicKey)
    );

    await sendAndConfirmTransaction(
        connection,
        transaction,
        [payerAccount, stickerFactoryAccount, stickerFactoryOwnerAccount],
        {
          commitment: 'singleGossip',
          preflightCommitment: 'singleGossip',
        },
      );
    return stickerFactoryAccount;
}

module.exports = {
    createStickerFactory
}