#!/bin/zsh

# Check if required arguments are provided
if [[ $# -lt 2 ]]; then
    echo "Usage: $0 <depth> <fen> [moves]"
    exit 1
fi

DEPTH="$1"
FEN="$2"
MOVES="${@:3}"

# Call your existing Rust program
# Assuming your program is set up to accept these command line arguments
RESULT=$(cargo run --release -- perft "$DEPTH" "$FEN" $MOVES 2>/tmp/perft_stderr.log)

echo "$RESULT" 
#| awk '
#/^[a-h][1-8][a-h][1-8]/ { 
#    moves[$1] = $2                  # Store move and its count in an array
#    print $1, $2                     # Print move and node count
#}
#END {
#    print ""                         # Add a blank line
#    total = 0                         # Initialize total nodes counter
#    for (move in moves) total += moves[move]  # Sum all counts
#3    if (total > 0) print total       # Print total nodes
#}'

