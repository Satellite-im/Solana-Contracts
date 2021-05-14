const BufferLayout = require("buffer-layout");

/// Address type input
const ADDRESS_TYPE_INPUT = BufferLayout.union(BufferLayout.u8("addressType"));
ADDRESS_TYPE_INPUT.addVariant(0, undefined, "artist");
ADDRESS_TYPE_INPUT.addVariant(1, undefined, "sticker");

const LAYOUT = BufferLayout.union(BufferLayout.u8("instruction"));
LAYOUT.addVariant(
  0,
  BufferLayout.struct([
    BufferLayout.seq(BufferLayout.u8(), 32, "name"),
    BufferLayout.seq(BufferLayout.u8(), 256, "signature"),
    BufferLayout.seq(BufferLayout.u8(), 256, "description"),
  ]),
  "registerArtist"
);

LAYOUT.addVariant(
  1,
  BufferLayout.struct([
    BufferLayout.nu64("max_supply"),
    BufferLayout.nu64("price"),
    BufferLayout.seq(BufferLayout.u8(), 256, "uri"),
    BufferLayout.seq(BufferLayout.u8(), 8, "symbol"),
    BufferLayout.seq(BufferLayout.u8(), 32, "name"),
  ]),
  "createNewSticker"
);

LAYOUT.addVariant(2, undefined, "createStickerFactory");

LAYOUT.addVariant(3, undefined, "buySticker");

LAYOUT.addVariant(
  4,
  BufferLayout.seq(BufferLayout.nu64(), 1),
  "changeStickerPrice"
);

LAYOUT.addVariant(5, ADDRESS_TYPE_INPUT, "createAccount");

const stickerFactoryAccountLayout = BufferLayout.struct([
  BufferLayout.nu64("artist_count"),
  BufferLayout.nu64("sticker_count"),
  BufferLayout.seq(BufferLayout.u8(), 32, "owner"),
]);

const artistAccountLayout = BufferLayout.struct([
  BufferLayout.seq(BufferLayout.u8(), 32, "user"),
  BufferLayout.seq(BufferLayout.u8(), 32, "user_token_acc"),
  BufferLayout.seq(BufferLayout.u8(), 32, "name"),
  BufferLayout.seq(BufferLayout.u8(), 256, "signature"),
  BufferLayout.seq(BufferLayout.u8(), 256, "description"),
]);

const tokenAccountLayout = BufferLayout.struct([
  BufferLayout.seq(BufferLayout.u8(), 32, "mint"),
  BufferLayout.seq(BufferLayout.u8(), 32, "owner"),
  BufferLayout.nu64("amount"),
  BufferLayout.u32("delegateOption"),
  BufferLayout.seq(BufferLayout.u8(), 32, "delegate"),
  BufferLayout.u8("state"),
  BufferLayout.u32("isNativeOption"),
  BufferLayout.nu64("is_native"),
  BufferLayout.nu64("delegated_amount"),
  BufferLayout.u32("closeAuthorityOption"),
  BufferLayout.seq(BufferLayout.u8(), 32, "close_authority"),
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
  stickerFactoryAccountLayout,
  artistAccountLayout,
  tokenAccountLayout,
};
