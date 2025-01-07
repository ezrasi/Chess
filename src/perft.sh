#!/bin/zsh


if [[ $# -lt 2 ]]; then
    echo "Usage: $0 <depth> <fen> [moves]"
    exit 1
fi

DEPTH="$1"
FEN="$2"
MOVES="${@:3}"

RESULT=$(cargo run --release -- perft "$DEPTH" "$FEN" $MOVES 2>/tmp/perft_stderr.log)

echo "$RESULT" 

