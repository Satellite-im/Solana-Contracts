const { Connection, clusterApiUrl, PublicKey } = require("@solana/web3.js");

const { getArtistAccountData } = require("./../client/sticker.js");

const NETWORK = clusterApiUrl("devnet");

const ARTIST_KEY = new PublicKey("W3HoXfKVPhVMGsvB79d7ywtzn4oQxY5xtXpwDyCaYYn");

(async function () {
  const connection = new Connection(NETWORK);

  let artistData = await getArtistAccountData(connection, ARTIST_KEY);

  console.log({
    user: new PublicKey(Buffer.from(artistData.user)).toBase58(),
    user_token_acc: new PublicKey(
      Buffer.from(artistData.user_token_acc)
    ).toBase58(),
    name: artistData.name,
    signature: artistData.signature,
    description: artistData.description,
  });
})();
