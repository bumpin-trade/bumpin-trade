#!/usr/bin/env bash


num_loops=10
echo "num_loops: $num_loops"
echo "Creating... "

for cmd in solana spl-token jq; do
  if ! command -v $cmd &> /dev/null; then
    echo "$cmd could not be found"
    exit 1
  fi
done

# Step 1: Airdrop SOL to the admin account
admin_pubkey=$(solana-keygen pubkey ~/admin.json)
if [ -z "$admin_pubkey" ]; then
  echo "Failed to get admin pubkey"
  exit 1
fi

solana airdrop 100000 "$admin_pubkey" --url localhost > /dev/null
if [ $? -ne 0 ]; then
  echo "Airdrop failed"
  exit 1
fi

results=""

for ((i=1; i<=num_loops; i++))
do
  # Step 2: Create a new SPL token and extract the token address
token=$(spl-token create-token --output json --url localhost | jq -r .commandOutput.address)
  if [ -z "$token" ]; then
    echo "Failed to create token"
    exit 1
  fi

  # Step 3: Create a new token account and extract the account address
  account_output=$(spl-token create-account "$token" --url localhost)
  account=$(echo "$account_output" | sed -n 's/Creating account \(.*\)/\1/p')
  if [ -z "$account" ]; then
    echo "Failed to create account"
    exit 1
  fi

  # Step 4: Mint tokens to the created account
  mint_amount=$((i * 1000000))
  spl-token mint "$token" "$mint_amount" "$account" --url localhost> /dev/null
  if [ $? -ne 0 ]; then
    echo "Minting tokens failed"
    exit 1
  fi

  results+="Token Mint: $token Token Account PublicKey: $account Amount: $mint_amount\n"
done

echo -e "$results"