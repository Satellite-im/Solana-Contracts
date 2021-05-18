function stringToBuffer(value, length) {
  return Buffer.concat(
    [Buffer.from(value, "utf-8"), Buffer.alloc(length, 0x00)],
    length
  );
}

async function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForAccount(connection, accountKey) {
  while (true) {
    const accountInfo = await connection.getAccountInfo(accountKey);
    if (accountInfo === null) {
      await sleep(3000);
      continue;
    } else {
      break;
    }
  }
}

module.exports = {
  stringToBuffer,
  waitForAccount,
};
