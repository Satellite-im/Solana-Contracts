const BufferLayout = require("buffer-layout");

const LAYOUT = BufferLayout.union(BufferLayout.u8("instruction"));

LAYOUT.addVariant(
  0,
  BufferLayout.struct([BufferLayout.seq(BufferLayout.seq(BufferLayout.u8(), 32), 4, "tex")]),
  "makeRequest"
);

LAYOUT.addVariant(
  1,
  BufferLayout.struct([BufferLayout.seq(BufferLayout.seq(BufferLayout.u8(), 32), 4, "tex")]),
  "acceptRequest"
);

LAYOUT.addVariant(
  2,
  undefined,
  "denyRequest"
);

LAYOUT.addVariant(
  3,
  undefined,
  "removeRequest"
);

LAYOUT.addVariant(
  4,
  undefined,
  "removeFriend"
);

LAYOUT.addVariant(
  5,
  BufferLayout.seq(BufferLayout.u8(), 32),
  "createAccount"
);

const friendLayout = BufferLayout.struct([
  BufferLayout.seq(BufferLayout.u8(), 32, "from"),
  BufferLayout.u8("status"),
  BufferLayout.seq(BufferLayout.u8(), 32, "to"),
  BufferLayout.seq(BufferLayout.u8(), 32, "textileFrom1"),
  BufferLayout.seq(BufferLayout.u8(), 32, "textileFrom2"),
  BufferLayout.seq(BufferLayout.u8(), 32, "textileFrom3"),
  BufferLayout.seq(BufferLayout.u8(), 32, "textileFrom4"),
  BufferLayout.seq(BufferLayout.u8(), 32, "textileTo1"),
  BufferLayout.seq(BufferLayout.u8(), 32, "textileTo2"),
  BufferLayout.seq(BufferLayout.u8(), 32, "textileTo3"),
  BufferLayout.seq(BufferLayout.u8(), 32, "textileTo4"),
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
  friendLayout,
};
