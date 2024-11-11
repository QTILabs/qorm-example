#!/bin/bash
examples=("integrate_with_sqlite" "sqlx-sqlite")

runCargoScript() {
    cargo fmt
    cargo check
    cargo clippy
    cargo test
}

index=1
for str in ${examples[@]}; do
    if [[ $index == 1 ]]; then
        cd "./${str}"
    else
        cd "../${str}"
    fi
    pwd
    runCargoScript
    index=$(($index + 1))
done
