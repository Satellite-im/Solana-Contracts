

function stringToBuffer(value, length) {
  return Buffer.concat([
      Buffer.from(value, 'utf-8'), 
      Buffer.alloc(length, 0x00)
    ], length)
}

module.exports = {
    stringToBuffer,
}