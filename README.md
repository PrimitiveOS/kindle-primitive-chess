<div align="center">

# Chess

**Play standard chess against your Kindle.**

[**Install guide**](INSTALL.md) ·
[**Device notes**](docs/device-notes.md) ·
[**Changelog**](CHANGELOG.md)

</div>

---

Tap a piece to see its legal moves, then tap a destination. The opponent plays locally using a bounded search that
evaluates material and control of the center.

## Screenshots

<table>
  <tr>
    <td align="center"><img src="docs/images/chess-moves.png" alt="Starting chess position with a pawn selected and both legal moves marked" width="260"><br><sub>Legal moves for the selected pawn</sub></td>
    <td align="center"><img src="docs/images/chess-dark.png" alt="Chess board in dark mode with a confirmation that the PGN was saved" width="260"><br><sub>Game position and PGN export in dark mode</sub></td>
  </tr>
</table>

<sub>Screenshots from the desktop simulator at 1264 × 1680, using a sample game.</sub>

## Highlights

- **Three opponent levels.** Choose Easy, Medium, or Hard.
- **Undo and resume.** Undo the previous turn or reopen a saved game.
- **PGN export.** Save the game to the Kindle's `documents/` folder.

## Quick start

After setting up PrimitiveOS, open **Store** from Primitive Launcher. Choose **Chess** under **Apps**,
tap **Get**, and open it from the Launcher grid. The [install guide](INSTALL.md) also covers local USB testing.

## Status

Desktop tests and previews passed on macOS, and the ARMHF package passed the local Store fixture.
On an ARMHF Kindle Oasis 3 running firmware 5.18.2, Chess showed the new-game confirmation, preserved the
original board after Cancel, and returned to Launcher. [Device notes](docs/device-notes.md)

## Develop locally

Keep the Primitive Tools and Launcher repositories beside this checkout. The desktop command writes PNG
previews and a sample PGN to `build/previews`.

```sh
cargo test --offline --locked --workspace
cargo run --offline --locked -p chess-desktop
```

## Licence

Copyright (C) 2026 Doan Nguyen.

- Original code is under the [PolyForm Noncommercial License 1.0.0](LICENSE).
- Original documentation and pictures, including these screenshots, are under [CC BY-NC-SA 4.0](LICENSE-DOCS).
- Standard chess move generation and notation use the MIT-licensed [cozy-chess](https://github.com/analog-hors/cozy-chess).
- The Noto Sans fonts supplied by Primitive Launcher retain their SIL Open Font License.
