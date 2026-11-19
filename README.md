# Primitive Chess

Play standard chess against the Kindle. Tap a piece to see its legal moves. Change difficulty, undo the previous turn,
resume a saved position, or export the game as PGN to `documents/`.

The original device opponent uses a bounded material and center search. Standard move generation and notation come
from the MIT-licensed [cozy-chess](https://github.com/analog-hors/cozy-chess).

Run `cargo test --offline --workspace` and `cargo run --offline -p chess-desktop` with the shared Tools and
Launcher repositories next to this one. The ARMHF package uses the launcher's fonts.

Source is PolyForm Noncommercial 1.0.0. Documentation is CC BY-NC-SA 4.0.
