#!/usr/bin/env bash

usage() {
    echo Usage:
    echo "$1 <srtool compressed runtime path>"
    echo "$2 <para_id>"
    echo "e.g.: ./scripts/create_coretime_kusama_spec.sh ./target/release/wbuild/coretime-kusama-runtime/coretime_kusama_runtime.compact.compressed.wasm 1005"
    exit 1
}

if [ -z "$1" ]; then
    usage
fi

if [ -z "$2" ]; then
    usage
fi

set -e

rt_path=$1
para_id=$2

echo "Generating chain spec for runtime: $rt_path and para_id: $para_id"

binary="./target/release/polkadot-parachain"

# build the chain spec we'll manipulate
$binary build-spec --chain coretime-kusama-dev > chain-spec-plain.json

# convert runtime to hex
cat $rt_path | od -A n -v -t x1 |  tr -d ' \n' > rt-hex.txt

# replace the runtime in the spec with the given runtime and set some values to production
# TODO: Get bootNodes, invulnerables, and session keys https://github.com/paritytech/devops/issues/2725
cat chain-spec-plain.json | jq --rawfile code rt-hex.txt '.genesis.runtime.system.code = ("0x" + $code)' \
    | jq '.name = "Kusama Coretime"' \
    | jq '.id = "coretime-kusama"' \
    | jq '.chainType = "Live"' \
    | jq '.bootNodes = [
                       ]' \
    | jq '.relay_chain = "kusama"' \
    | jq --argjson para_id $para_id '.para_id = $para_id' \
    | jq --argjson para_id $para_id '.genesis.runtime.parachainInfo.parachainId = $para_id' \
    | jq '.genesis.runtime.balances.balances = []' \
    | jq '.genesis.runtime.collatorSelection.invulnerables = [
                                                             ]' \
    | jq '.genesis.runtime.session.keys = [
                                          ]' \
    > edited-chain-spec-plain.json

# build a raw spec
$binary build-spec --chain edited-chain-spec-plain.json --raw > chain-spec-raw.json
cp edited-chain-spec-plain.json coretime-kusama-spec.json
cp chain-spec-raw.json ./parachains/chain-specs/coretime-kusama.json
cp chain-spec-raw.json coretime-kusama-spec-raw.json

# build genesis data
$binary export-genesis-state --chain chain-spec-raw.json > coretime-kusama-genesis-head-data

# build genesis wasm
$binary export-genesis-wasm --chain chain-spec-raw.json > coretime-kusama-wasm