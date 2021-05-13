const {
    SystemProgram,
    Transaction,
    TransactionInstruction,
    PublicKey,
    sendAndConfirmTransaction,
    SYSVAR_RENT_PUBKEY,
} = require('@solana/web3.js');

const {
    encodeInstructionData,
    friendInfoAccountLayout,
} = require('./../client/layout.js');

const FRIENDS_PROGRAM_ID = new PublicKey(
    '92k8fHjwZV1tzFhokS1NoyLz65vhz3E3VdEcghXF4GRr'
);

const FRIEND_INFO_SEED = 'friendinfo';

async function createDerivedAccount(connection, payerAccount, seedKey, seedString, params) {
    let base = await PublicKey.findProgramAddress([seedKey.toBytes()], FRIENDS_PROGRAM_ID);
    let addressToCreate = await PublicKey.createWithSeed(base[0], seedString, FRIENDS_PROGRAM_ID);
    let instruction = new TransactionInstruction({
      keys: [
        {pubkey: payerAccount.publicKey, isSigner: true, isWritable: true},
        {pubkey: seedKey, isSigner: false, isWritable: false},
        {pubkey: base[0], isSigner: false, isWritable: false},
        {pubkey: addressToCreate, isSigner: false, isWritable: true},
        {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
        {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
      ],
      programId: FRIENDS_PROGRAM_ID,
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

async function initFriendInfo(friendInfoPubKey, userKey) {
    return new TransactionInstruction({
        keys: [
          {pubkey: friendInfoPubKey, isSigner: false, isWritable: true},
          {pubkey: userKey, isSigner: true, isWritable: false},
          {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
        ],
        programId: FRIENDS_PROGRAM_ID,
        data:
          encodeInstructionData({
            initFriendInfo: {},
          })
      });
}

async function createFriendInfo(connection, payerAccount, userAccount) {
    let params = {createAccount: {friendInfo: {}}};
    let friendInfoKey = await createDerivedAccount(connection, payerAccount, userAccount.publicKey, FRIEND_INFO_SEED, params);
    
    let transaction = new Transaction().add(await initFriendInfo(friendInfoKey, userAccount.publicKey));

    await sendAndConfirmTransaction(
        connection,
        transaction,
        [payerAccount, userAccount],
        {
          commitment: 'singleGossip',
          preflightCommitment: 'singleGossip',
        },
      );
      return friendInfoKey;
}

async function getFriendInfo(connection, friendInfoKey) {
    const accountInfo = await connection.getAccountInfo(friendInfoKey);
    if (accountInfo === null) {
        throw 'Error: cannot find the account';
    }
    const info = friendInfoAccountLayout.decode(Buffer.from(accountInfo.data));
    return info;
}

module.exports = {
    createFriendInfo,
    getFriendInfo,
}