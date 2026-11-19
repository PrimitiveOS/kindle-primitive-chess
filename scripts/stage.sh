#!/bin/sh
# stage.sh TARGET [STAGE_DIR]
set -eu
cd "$(dirname "$0")/.."
case "${1:-armhf}" in
    armhf|armv7-unknown-linux-musleabihf) target=armv7-unknown-linux-musleabihf ;;
    *) echo "Unsupported package architecture: $1" >&2; exit 1 ;;
esac
stage=${2:-build/stage}
target_dir=${CARGO_TARGET_DIR:-target}
bin=$target_dir/$target/release/chess-kindle
[ -x "$bin" ] || { echo "Build $bin before staging." >&2; exit 1; }
install -d "$stage/chess/bin" "$stage/documents"
install -m 755 "$bin" "$stage/chess/bin/chess-kindle"
install -m 755 deploy/Chess.sh "$stage/documents/Chess.sh"
install -d "$stage/chess/licenses"
install -m 644 licenses/cozy-chess.txt "$stage/chess/licenses/cozy-chess.txt"
