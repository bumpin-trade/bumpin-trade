#!/usr/bin/env bash


num_loops=6
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
keeper_pubkey=$(solana-keygen pubkey ./keys/keeper.json)
reward_pubkey=$(solana-keygen pubkey ./keys/reward.json)
player_pubkey=$(solana-keygen pubkey ./keys/player.json)
player2_pubkey=$(solana-keygen pubkey ./keys/player2.json)
player3_pubkey=$(solana-keygen pubkey ./keys/player3.json)
player4_pubkey=$(solana-keygen pubkey ./keys/player4.json)
player5_pubkey=$(solana-keygen pubkey ./keys/player5.json)
if [ -z "$admin_pubkey" ]; then
  echo "Failed to get admin pubkey"
  exit 1
fi

solana airdrop 100000 "$admin_pubkey" --url localhost > /dev/null
solana airdrop 100000 "$keeper_pubkey" --url localhost > /dev/null
solana airdrop 100000 "$reward_pubkey" --url localhost > /dev/null
solana airdrop 100000 "$player_pubkey" --url localhost > /dev/null
solana airdrop 100000 "$player2_pubkey" --url localhost > /dev/null
solana airdrop 100000 "$player3_pubkey" --url localhost > /dev/null
solana airdrop 100000 "$player4_pubkey" --url localhost > /dev/null
solana airdrop 100000 "$player5_pubkey" --url localhost > /dev/null


# SOL USDC WBTC WHETH BONK BNB
decimals=(9 6 8 8 8 5)
mint_amounts=(1000 1000000 10 100 1000000000 1000)

results="================================================\n"

for ((i=1; i<=num_loops; i++))
do
  decimals_value=${decimals[$((i-1))]}
  mint_amount_value=${mint_amounts[$((i-1))]}

  # Step 2: Create a new SPL token and extract the token address
token=$(spl-token create-token --decimals "$decimals_value" --output json --url localhost | jq -r .commandOutput.address)
  if [ -z "$token" ]; then
    echo "Failed to create token"
    exit 1
  fi

  # Step 3: Create a new token account and extract the account address
  account_output=$(spl-token create-account "$token" --url localhost)
  account=$(echo "$account_output" | sed -n 's/Creating account \(.*\)/\1/p')

  account_player2_output=$(spl-token create-account "$token" --owner ./keys/player2.json --url localhost)
  account2=$(echo "$account_player2_output" | sed -n 's/Creating account \(.*\)/\1/p')

  account_player3_output=$(spl-token create-account "$token" --owner ./keys/player3.json --url localhost)
  account3=$(echo "$account_player3_output" | sed -n 's/Creating account \(.*\)/\1/p')

  account_player4_output=$(spl-token create-account "$token" --owner ./keys/player4.json --url localhost)
  account4=$(echo "$account_player4_output" | sed -n 's/Creating account \(.*\)/\1/p')

  account_player5_output=$(spl-token create-account "$token" --owner ./keys/player5.json --url localhost)
  account5=$(echo "$account_player5_output" | sed -n 's/Creating account \(.*\)/\1/p')

  account_reward_output=$(spl-token create-account "$token" --owner ./keys/reward.json --url localhost)
  account_reward=$(echo "$account_reward_output" | sed -n 's/Creating account \(.*\)/\1/p')

  # Step 4: Mint tokens to the created account
#  mint_amount=$((i * 100000))
  spl-token mint "$token" "$mint_amount_value" "$account" --url localhost> /dev/null
  spl-token mint "$token" "$mint_amount_value" "$account2" --url localhost> /dev/null
  spl-token mint "$token" "$mint_amount_value" "$account3" --url localhost> /dev/null
  spl-token mint "$token" "$mint_amount_value" "$account4" --url localhost> /dev/null
  spl-token mint "$token" "$mint_amount_value" "$account5" --url localhost> /dev/null

  results+="Token Mint: $token\nToken Account PublicKey: $account\nAmount: $mint_amount_value\nDecimals: $decimals_value\nReward Token Account: $account_reward\n================================================\n"

done

echo -e "$results"