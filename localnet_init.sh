#!/usr/bin/env sh

# ./localnet_init.sh 3  创建3个token和账户，并且为其铸币
num_loops="$1"
echo "num_loops: $num_loops"
echo "Creating... "
# Step 1: Airdrop SOL to the admin account
solana airdrop 10000 $(solana-keygen pubkey ./keys/admin.json) --url localhost> /dev/null
decimals=(9 6 8 8 5)
results=""

for ((i=1; i<=num_loops; i++))
do
  decimals_value=${decimals[$((i-1))]}
  # Step 2: Create a new SPL token and extract the token address
  token=$(spl-token create-token --decimals "$decimals_value" --output json | jq -r .commandOutput.address)

  # Step 3: Create a new token account and extract the account address
  account_output=$(spl-token create-account $token)
  account=$(echo "$account_output" | sed -n 's/Creating account \(.*\)/\1/p')

  # Step 4: Mint tokens to the created account
  mint_amount=$((i * 1000000))
  spl-token mint $token $mint_amount $account > /dev/null

  results+="Token Mint: $token Token Account PublicKey: $account Amount: $mint_amount Decimals: $decimals_value\n"
done

echo  "$results"
