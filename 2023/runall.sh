#!/bin/sh

inputs="input-example input-actual"

echo "========== CHECK =========="
cargo check --release

echo "========== CLIPPY =========="
cargo clippy --release

echo "========== TESTS =========="
cargo test --release
#for day in day*
#do
#    echo cargo test --bin "$day" --release
#    cargo test --bin "$day" --release
#    echo "--------------------"
#done

if [ "$1" = "-v" ]; then
    for input in $inputs
    do
        echo "========== INPUT FILES =========="
        for day in day*
        do
            if [ -f "$day/$input" ]; then
                echo cargo run --bin "$day" --release -- -i "$day/$input" -t
                cargo run --bin "$day" --release -- -i "$day/$input" -t
                echo "--------------------"
            fi
        done
    done
fi
