# Trustworthy proof

## Build contracts & sdk

`cwtools build && cwtools genjs .`

## Deploy on mainnet

create .env file from .env.example

`cwtools wasm deploy artifacts/trustworthy-proof.wasm --input '{}' --label 'trustworthy proof'`

## Update proof

`NODE_PATH=node_modules cwtools script scripts/update_proof.ts --address [CONTRACT_ADDRESS]`
