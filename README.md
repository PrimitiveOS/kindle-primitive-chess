<div align="center">

# Chess

**Play chess against your Kindle**

[![CI](https://img.shields.io/github/actions/workflow/status/PrimitiveOS/kindle-primitive-chess/ci.yml?branch=main&label=CI)](https://github.com/PrimitiveOS/kindle-primitive-chess/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/PrimitiveOS/kindle-primitive-chess?label=release)](https://github.com/PrimitiveOS/kindle-primitive-chess/releases)
[![Tag](https://img.shields.io/github/v/tag/PrimitiveOS/kindle-primitive-chess?label=tag)](https://github.com/PrimitiveOS/kindle-primitive-chess/tags)
[![Licence: PolyForm Noncommercial 1.0.0](https://img.shields.io/badge/licence-PolyForm%20Noncommercial%201.0.0-blue)](LICENSE)
[![Stars](https://img.shields.io/github/stars/PrimitiveOS/kindle-primitive-chess?style=social)](https://github.com/PrimitiveOS/kindle-primitive-chess/stargazers)
[![Store downloads](https://img.shields.io/endpoint?url=https%3A%2F%2Fprimitiveos.github.io%2Fkindle-primitive-catalog%2Fv1%2Fbadges%2Fprimitive-chess.json)](https://primitiveos.github.io/kindle-primitive-catalog/v1/index.json)

[**Download**](https://github.com/PrimitiveOS/kindle-primitive-chess/releases/latest) · [**Install guide**](INSTALL.md) · [**Changelog**](CHANGELOG.md)

</div>

---

Play standard chess at three levels, save a game and export PGN.

## Quick start

After setting up PrimitiveOS, open **Store** from Primitive Launcher. Choose **Chess** under **Apps**,
tap **Get**, and open it from the Launcher grid. The [install guide](INSTALL.md) also covers the standalone USB package.

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

## Develop locally

Keep the Primitive Tools and Launcher repositories beside this checkout. The desktop command writes PNG
previews and a sample PGN to `build/previews`.

```sh
cargo test --offline --locked --workspace
cargo run --offline --locked -p chess-desktop
```

## Status

| Device | Status |
|---|---|
| Kindle Oasis 3 (1264 × 1680, firmware 5.18.2) | Tested on hardware |

**Current version: 0.1.1.** Changes are in [CHANGELOG.md](CHANGELOG.md).

## Downloads

The badge at the top shows the number of Store downloads. It counts GitHub downloads of this app's release packages, across all versions and architectures, and is refreshed daily from the [Store catalog](https://primitiveos.github.io/kindle-primitive-catalog/v1/index.json).

## Documentation

[Install guide](INSTALL.md) · [Changelog](CHANGELOG.md) · [Device notes](docs/device-notes.md)

## Licence

Copyright (C) 2026 Doan Nguyen.

- Original code is under the [PolyForm Noncommercial License 1.0.0](LICENSE).
- Original documentation and pictures, including these screenshots, are under [CC BY-NC-SA 4.0](LICENSE-DOCS).
- Standard chess move generation and notation use the MIT-licensed [cozy-chess](https://github.com/analog-hors/cozy-chess).
- The Noto Sans fonts supplied by Primitive Launcher retain their SIL Open Font License.
