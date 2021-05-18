const BufferLayout = require("buffer-layout");

/// Address type input
const ADDRESS_TYPE_INPUT = BufferLayout.union(BufferLayout.u8("addressType"));
ADDRESS_TYPE_INPUT.addVariant(0, undefined, "friendInfo");
ADDRESS_TYPE_INPUT.addVariant(1, BufferLayout.nu64("index"), "requestOutgoing");
ADDRESS_TYPE_INPUT.addVariant(2, BufferLayout.nu64("index"), "requestIncoming");
ADDRESS_TYPE_INPUT.addVariant(
  3,
  BufferLayout.seq(BufferLayout.u8(), 32, "key"),
  "friend"
);

const LAYOUT = BufferLayout.union(BufferLayout.u8("instruction"));
LAYOUT.addVariant(0, undefined, "initFriendInfo");

LAYOUT.addVariant(1, undefined, "makeRequest");

LAYOUT.addVariant(
  2,
  BufferLayout.seq(BufferLayout.seq(BufferLayout.u8(), 32), 2),
  "acceptRequest"
);

LAYOUT.addVariant(3, undefined, "denyRequest");

LAYOUT.addVariant(4, undefined, "removeRequest");

LAYOUT.addVariant(5, undefined, "removeFriend");

LAYOUT.addVariant(6, ADDRESS_TYPE_INPUT, "createAccount");

const friendInfoAccountLayout = BufferLayout.struct([
  BufferLayout.nu64("requests_incoming"),
  BufferLayout.nu64("requests_outgoing"),
  BufferLayout.nu64("friends"),
  BufferLayout.seq(BufferLayout.u8(), 32, "user"),
]);

const requestAccountLayout = BufferLayout.struct([
  BufferLayout.seq(BufferLayout.u8(), 32, "from"),
  BufferLayout.seq(BufferLayout.u8(), 32, "to"),
]);

const instructionMaxSpan = Math.max(
  ...Object.values(LAYOUT.registry).map((r) => r.span)
);

function encodeInstructionData(instruction) {
  let b = Buffer.alloc(instructionMaxSpan);
  let span = LAYOUT.encode(instruction, b);
  return b.slice(0, span);
}

module.exports = {
  LAYOUT,
  encodeInstructionData,
  ADDRESS_TYPE_INPUT,
  friendInfoAccountLayout,
  requestAccountLayout,
};
