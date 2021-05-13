const {
    Connection,
    Account,
    clusterApiUrl,
    PublicKey,
  } = require('@solana/web3.js');
  const {
    createDweller,
    createServer,
  } = require('./../client/server.js');
  const {
    dwellerAccountLayout
  } = require('./../client/layout.js');
  
  const NETWORK = clusterApiUrl('devnet');
  const fs = require('fs');
  const keyPath = 'test_wallet.json';
  const pk = JSON.parse(fs.readFileSync(keyPath));
  const PAYER_ACCOUNT = new Account(pk);
  
  async function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
  
  (async function() {
  
      const connection = new Connection(NETWORK);
  
      const dwellerAccount = await createDweller(connection, PAYER_ACCOUNT, "test_name");

      while (true) {  // wait till block are finalized and we can use new dwellerAccount
        await sleep(3000);
        const accountInfo = await connection.getAccountInfo(dwellerAccount.publicKey);
        if (accountInfo === null) {
            continue;
        } else {
            break;
        }
      }

      let dwellerServer = await createServer(connection, PAYER_ACCOUNT, dwellerAccount, "test_name");
      console.log("And here is the new dweller server: ", dwellerServer.publicKey.toBase58());
  }())