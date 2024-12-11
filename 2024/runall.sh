#!/bin/sh

inputs="input-example input-actual"

echo "========== CHECK =========="
cargo check --release

echo "========== CLIPPY =========="
cargo clippy --release

echo "========== TESTS =========="
cargo test --release

if [ "$1" = "-v" ]; then
    echo "========== INPUT FILES =========="
    for day in day*
    do
        for input in $inputs
        do
            if [ -f "$day/$input" ]; then
                echo cargo run --bin "$day" --release -- -i "$day/$input" -t
                cargo run --bin "$day" --release -- -i "$day/$input" -t
                echo "--------------------"
            fi
        done
    done
fi
