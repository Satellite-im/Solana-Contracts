const {
  Connection,
  Account,
  clusterApiUrl,
} = require('@solana/web3.js');
const {
  createDweller
} = require('./../client/server.js');

const NETWORK = clusterApiUrl('devnet');
const fs = require('fs');
const keyPath = 'test_wallet.json';
const pk = JSON.parse(fs.readFileSync(keyPath));
const PAYER_ACCOUNT = new Account(pk);

(async function() {

    const connection = new Connection(NETWORK);

    const dwellerAccount = await createDweller(connection, PAYER_ACCOUNT, "test_name");

    console.log(`Dweller created with pubkey ${dwellerAccount.publicKey}`);

}())