#!/usr/bin/env sh

num_loops="$1"
echo "num_loops: $num_loops"
echo "Creating... "
# Step 1: Airdrop SOL to the admin account
solana airdrop 100000 > /dev/null
solana airdrop 100000 $(solana-keygen pubkey ./keys/admin.json) --url localhost> /dev/null
solana airdrop 100000 $(solana-keygen pubkey ./keys/keeper.json) --url localhost> /dev/null
solana airdrop 100000 $(solana-keygen pubkey ./keys/player.json) --url localhost> /dev/null
solana airdrop 100000 $(solana-keygen pubkey ./keys/player2.json) --url localhost> /dev/null
solana airdrop 100000 $(solana-keygen pubkey ./keys/player3.json) --url localhost> /dev/null
solana airdrop 100000 $(solana-keygen pubkey ./keys/player4.json) --url localhost> /dev/null
solana airdrop 100000 $(solana-keygen pubkey ./keys/player5.json) --url localhost> /dev/null
solana airdrop 100000 $(solana-keygen pubkey ./keys/reward.json) --url localhost> /dev/null
coin_names=("stable" "btc" "eth")
decimals=(8 8 8)
mint_amounts=(1000000 100 1000)
results="================================================\n"
results_code="\n================================================\n"
results_vault_code="\n================================================\n"
for ((i=1; i<=num_loops; i++))
do
  coin_name=${coin_names[$((i-1))]}
  decimals_value=${decimals[$((i-1))]}
  mint_amount_value=${mint_amounts[$((i-1))]}
  # Step 2: Create a new SPL token and extract the token address
  token=$(spl-token create-token --decimals "$decimals_value" --output json | jq -r .commandOutput.address)

  # Step 3: Create a new token account and extract the account address
  account_output=$(spl-token create-account $token)
  account=$(echo "$account_output" | sed -n 's/Creating account \(.*\)/\1/p')
  account_player2_output=$(spl-token create-account $token --owner ./keys/player2.json)
  account2=$(echo "$account_player2_output" | sed -n 's/Creating account \(.*\)/\1/p')
  account_player3_output=$(spl-token create-account $token --owner ./keys/player3.json)
  account3=$(echo "$account_player3_output" | sed -n 's/Creating account \(.*\)/\1/p')
  account_player4_output=$(spl-token create-account $token --owner ./keys/player4.json)
  account4=$(echo "$account_player4_output" | sed -n 's/Creating account \(.*\)/\1/p')
  account_player5_output=$(spl-token create-account $token --owner ./keys/player5.json)
  account5=$(echo "$account_player5_output" | sed -n 's/Creating account \(.*\)/\1/p')
  account_reward_output=$(spl-token create-account $token --owner ./keys/reward.json)
  account_reward=$(echo "$account_reward_output" | sed -n 's/Creating account \(.*\)/\1/p')


  # Step 4: Mint tokens to the created account
  spl-token mint $token $mint_amount_value $account > /dev/null
  spl-token mint $token $mint_amount_value $account2 > /dev/null
  spl-token mint $token $mint_amount_value $account3 > /dev/null
  spl-token mint $token $mint_amount_value $account4 > /dev/null
  spl-token mint $token $mint_amount_value $account5 > /dev/null

  results+="Token Mint: $token  Amount: $mint_amount  Decimals: $decimals_value  Reward Token Account: $account_reward\n"
  results_code+="${coin_name}CoinMint = \"$token\";\n"
  results_vault_code+="${coin_name}CoinDaoVault = \"$account_reward\";\n"
done

##create wsol reward_vault
spl-token create-account So11111111111111111111111111111111111111112 --owner ./keys/reward.json

echo  "$results"
echo  "$results_code"
echo  "$results_vault_code"