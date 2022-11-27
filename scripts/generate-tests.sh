#!/usr/bin/env bash

export CORN_TEST=bar

rm -rf assets/outputs
mkdir -p assets/outputs/{json,yaml,toml}

for file in assets/inputs/*; do
  basename=$(basename "$file" .corn)

  echo "$basename"

  cargo run -- "$file" -t json > assets/outputs/json/"$basename".json
  cargo run -- "$file" -t yaml > assets/outputs/yaml/"$basename".yml
  cargo run -- "$file" -t toml > assets/outputs/toml/"$basename".toml
done